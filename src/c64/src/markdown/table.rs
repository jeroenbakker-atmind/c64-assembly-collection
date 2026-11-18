#[derive(Clone, Copy)]
pub enum ColumnAlign {
    Left,
    Right,
}

/// A Markdown table that pads its columns, so the raw output is readable in a terminal too.
pub struct MarkdownTable {
    headers: Vec<String>,
    aligns: Vec<ColumnAlign>,
    rows: Vec<Vec<String>>,
}

impl MarkdownTable {
    pub fn new(headers: &[&str], aligns: &[ColumnAlign]) -> Self {
        assert_eq!(headers.len(), aligns.len());
        Self {
            headers: headers.iter().map(|header| (*header).to_string()).collect(),
            aligns: aligns.to_vec(),
            rows: Vec::new(),
        }
    }

    pub fn add_row<S, I>(&mut self, cells: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let row = cells.into_iter().map(Into::into).collect::<Vec<_>>();
        assert_eq!(row.len(), self.headers.len());
        self.rows.push(row);
    }

    pub fn print(&self) {
        let widths = self.column_widths();
        Self::print_row(&self.headers, &widths, &self.aligns);
        Self::print_separator(&widths, &self.aligns);
        for row in &self.rows {
            Self::print_row(row, &widths, &self.aligns);
        }
    }

    fn column_widths(&self) -> Vec<usize> {
        let mut widths = self
            .headers
            .iter()
            .map(|header| header.chars().count().max(4))
            .collect::<Vec<_>>();
        for row in &self.rows {
            for (index, cell) in row.iter().enumerate() {
                widths[index] = widths[index].max(cell.chars().count()).max(4);
            }
        }
        widths
    }

    fn print_row(cells: &[String], widths: &[usize], aligns: &[ColumnAlign]) {
        print!("|");
        for index in 0..cells.len() {
            let padding = " ".repeat(widths[index] - cells[index].chars().count());
            match aligns[index] {
                ColumnAlign::Left => print!(" {}{} |", cells[index], padding),
                ColumnAlign::Right => print!(" {}{} |", padding, cells[index]),
            }
        }
        println!();
    }

    fn print_separator(widths: &[usize], aligns: &[ColumnAlign]) {
        print!("|");
        for (width, align) in widths.iter().zip(aligns) {
            let rule = "-".repeat(width - 1);
            match align {
                ColumnAlign::Left => print!(" :{} |", rule),
                ColumnAlign::Right => print!(" {}: |", rule),
            }
        }
        println!();
    }
}
