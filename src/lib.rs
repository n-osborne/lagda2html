use std::io::{ BufReader, BufRead };
use std::fs::File;

pub fn mk_html(br : BufReader<File>) -> String {
    let mut res = String::new();
    res.push_str(&mk_header());
    res.push_str(&parse(br));
    res.push_str(&close_doc());
    res
}


fn open_code() -> String {
    String::from("<div>\n<pre><code>\n")
}

fn close_code() -> String {
    String::from("</code></pre></div>\n")
}

fn mk_header() -> String {
    String::from("<!DOCTYPE html>\n\
                  <html lang=\"en\">\n\
                  <head>\n\
                  \t<meta charset=\"utf-8\">\
                  \t<link rel=\"stylesheet\" href=\"https://latex.now.sh/style.css\">\n\
                  </head>\n\
                  <body>\n")
}

fn close_doc() -> String {
    String::from("</body>\n</html>")
}

fn parse_line_of_text(s : String) -> String {
    let mut res = String::new();
    let mut code = false;
    let mut emph = false;
    for c in s.chars() {
        match c {
            '`' =>
                if code {
                    res.push_str("</code>");
                    code = false;
                } else {
                    res.push_str("<code>");
                    code = true;
                }
            '*' =>
                if emph {
                    res.push_str("</em>");
                    emph = false;
                } else {
                    res.push_str("<em>");
                    emph = true;
                }
            x => res.push(x)
        }
    }
    res.push('\n');
    res
}

fn parse_line_of_code(s: String) -> String {
    let mut res = String::new();
    res.push_str("<span>");
    res.push_str(&s);
    res.push_str("</span>\n");
    res
}

fn mk_title(s : String) -> String {
    let mut res = String::new();
    res.push_str("<H1>");
    res.push_str(&parse_line_of_text(s));
    res.push_str("</H1>");
    res
}

fn parse(br : BufReader<File>) -> String {
    let mut res = String::new();
    let mut reading_text = false;
    let mut reading_code = false;
    let mut title = true;
    for line in br.lines() {
        if let Ok(line) = line {
            if title {
                res.push_str(&mk_title(line));
                title = false;
            } else {
                match line.as_str() {
                    "```agda" => {
                        res.push_str(&open_code());
                        reading_code = true;
                    },
                    "```" => {
                        res.push_str(&close_code());
                    reading_code = false;
                    },
                    "" =>
                        if reading_text {
                            res.push_str("</p>\n");
                            reading_text = false;
                        },
                    &_ =>       
                        if reading_text {
                            res.push_str(&parse_line_of_text(line));
                        } else if reading_code {
                            res.push_str(&parse_line_of_code(line));
                        } else {
                            res.push_str("<p>\n");
                            res.push_str(&parse_line_of_text(line));
                            reading_text = true;
                        }
                }
            }
        }
    }

    res
}
