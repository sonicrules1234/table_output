use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Table {
    num_cols: usize,
    rows: Vec<Vec<String>>,
}

#[derive(Clone, Debug)]
pub enum TableError {
    WrongNumberCols,
    NotEnoughCols,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Self {
        let mut rows  = Vec::new();
        let cols = headers.len();
        rows.push(headers);
        Self {
            num_cols: cols,
            rows: rows,
        }
    }
    pub fn add_row(&mut self, row: Vec<String>) -> Result<(), TableError> {
        if row.len() == self.num_cols {
            self.rows.push(row);
            return Ok(());
        }
        return Err(TableError::WrongNumberCols);
    }
    pub fn output_csv(&self) -> String {
        let mut wtr = csv::Writer::from_writer(vec![]);
        for row in self.rows.clone() {
            wtr.write_record(row.as_slice()).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    }
    pub fn output_json(&self) -> String {
        let mut json_vec: Vec<HashMap<String, String>> = Vec::new();
        let headers = self.rows[0].clone();
        for row in self.rows.clone()[1..].iter() {
            let mut json_hashmap: HashMap<String, String> = HashMap::new();
            for spot_header in headers.iter().enumerate() {
                json_hashmap.insert(spot_header.1.clone(), row[spot_header.0].clone());
            }
            json_vec.push(json_hashmap);
        }
        serde_json::to_string_pretty(&json_vec).unwrap()
    }
    pub fn output_table_html(&self, name: Option<impl Into<String>>) -> String {
        let mut data = String::new();
        if let Some(table_name) = name {
            data.push_str(format!("<p>{}</p>\n", html_escape::encode_text(table_name.into().as_str())).as_str())
        }
        data.push_str("<table>\n");
        data.push_str(format!("<th>{}</th>", self.rows[0].iter().map(|x| html_escape::encode_text(x).to_string()).collect::<Vec<String>>().join("</th><th>")).as_str());
        data.push_str("\n");
        for row in self.rows[1..].iter() {
            data.push_str("<tr>\n");
            data.push_str(format!("<td>\n{}</td>", row.iter().map(|x| html_escape::encode_text(x).to_string()).collect::<Vec<String>>().join("</td>\n<td>")).as_str());
            data.push_str("\n</tr>\n");
        }
        data.push_str("</table>\n");
        data
    }
    fn create_line(input: Vec<String>, wrap_space: usize) -> String {
        let mut line = String::new();
        line.push_str("|");
        //let spacing = wrap_space
        for in_string in input {
            //println!("'{}'", in_string);
            line.push_str(format!(" {}{}|", in_string, vec![" "; wrap_space + 1 - in_string.chars().count()].join("")).as_str());
        }
        line.push('\n');
        line
    }
    pub fn output_pretty_table(&self, width: Option<usize>) -> Result<String, TableError> {
        let mut data = String::new();
        let term_cols: usize;
        if let Some(w) = width {
            term_cols = w;
        } else if let Some(term_size) = termsize::get() {
            term_cols = term_size.cols.into();
        } else {
            term_cols = 200;
        }
        let headers = self.rows[0].clone();
        let mut total_chars: usize = (headers.iter().map(|x| x.chars().count() + 3).max().unwrap()) * headers.len() + 1;
        if total_chars > term_cols {
            return Err(TableError::NotEnoughCols);
        }
        let mut test_add = 1;
        while (headers.iter().map(|x| x.chars().count() + 3 + test_add).max().unwrap()) * headers.len() + 1 <= term_cols {
            total_chars = (headers.iter().map(|x| x.chars().count() + 3 + test_add).max().unwrap()) * headers.len() + 1;
            test_add += 1
        }
        //println!("termcols {} totalchars {}", term_cols, total_chars);
        //println!("{}", total_chars);
        let spacing: usize = (total_chars / headers.len()) - 1;
        //assert_eq!(spacing * headers.len(), total_chars)
        //println!("{}", spacing);
        let wrap_space = spacing - 2;
        //println!("{}", wrap_space);
        
        //let mut lines: Vec<String> = Vec::new();
        let mut horiz_border = String::new();
        horiz_border.push('+');
        for _count in 0..headers.len() {
            for _count2 in 0..spacing {
                horiz_border.push('=');
            }
            horiz_border.push('+');
        }
        horiz_border.push('\n');
        data.push_str(horiz_border.as_str());
        data.push_str(Self::create_line(headers, wrap_space).as_str());
        data.push_str(horiz_border.as_str());
        horiz_border = horiz_border.replace("=", "-");
        for row in self.rows[1..].iter() {
            let mut max_num_lines = 1;
            let mut vals_lines_vec: Vec<Vec<String>> = Vec::new();
            for val in row {
                let lines = textwrap::fill(val, wrap_space).replace("\r", "").split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
                let num = lines.len();
                vals_lines_vec.push(lines);
                if num > max_num_lines {
                    max_num_lines = num;
                }
            }
            for count in 0..max_num_lines {
                //let mut spots: Vec<String> = Vec::new();
                let val_lines_data: Vec<String> = vals_lines_vec.iter().map(|x| {
                    if x.len() - 1 < count {
                        String::new()
                    } else {
                        x[count].clone()
                    }
                }).collect();
                data.push_str(Self::create_line(val_lines_data, wrap_space).as_str());
            }
            data.push_str(horiz_border.as_str());
        }
        //data.push_str(horiz_border.as_str());
        Ok(data)
    }
}