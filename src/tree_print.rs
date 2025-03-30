use crate::bin_tree::{BinTree, BinTreeNode};
use rt_format::{Format, FormatArgument, ParsedFormat, Specifier};
use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

#[allow(dead_code)]
const H_LINE: &str = "─";
#[allow(dead_code)]
const V_LINE: &str = "│";

#[allow(dead_code)]
const RIGHT_TOP: &str = "└";
#[allow(dead_code)]
const LEFT_TOP: &str = "┘";
#[allow(dead_code)]
const RIGHT_BOTTOM: &str = "┌";
#[allow(dead_code)]
const LEFT_BOTTOM: &str = "┐";

#[allow(dead_code)]
const RIGHT_T: &str = "├";
#[allow(dead_code)]
const LEFT_T: &str = "┤";
#[allow(dead_code)]
const TOP_T: &str = "┴";
#[allow(dead_code)]
const BOTTOM_T: &str = "┬";
#[allow(dead_code)]
const CROSS: &str = "┼";

struct Drawing {
    lines: Vec<String>,
    /// Width of the drawing in characters
    width: usize,
    /// Column index of the root node's center
    root_col: usize,
}

pub struct DynDisplay<'a> {
    inner: &'a dyn fmt::Display,
}

impl<'a> DynDisplay<'a> {
    pub fn new(inner: &'a dyn fmt::Display) -> Self {
        DynDisplay { inner }
    }
}

impl<'a> FormatArgument for DynDisplay<'a> {
    fn supports_format(&self, spec: &Specifier) -> bool {
        matches!(spec.format, Format::Display)
    }

    fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self.inner, f)
    }

    fn fmt_debug(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }

    // For unsupported formatting options, return an error.
    fn fmt_octal(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
    fn fmt_lower_hex(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
    fn fmt_upper_hex(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
    fn fmt_binary(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
    fn fmt_lower_exp(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
    fn fmt_upper_exp(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
    fn to_usize(&self) -> Result<usize, ()> {
        Err(())
    }
}

impl<T: Display> BinTree<T> {
    /// Formats the tree into a String using box characters.
    pub fn format_tree(&self, ext_format_str: Option<String>) -> String {
        if self.root.value.is_none() {
            return "(Empty Tree)".to_string();
        }
        Self::draw_subtree(&self.root, &ext_format_str)
            .map(|d| d.lines.join("\n"))
            .unwrap_or_else(|| "".to_string())
    }

    /// Recursive function to draw a subtree.
    fn draw_subtree(node: &BinTreeNode<T>, ext_format_str: &Option<String>) -> Option<Drawing> {
        // Get node representation
        let node_str = match &node.value {
            Some(val) => match ext_format_str {
                None => val.to_string(),
                Some(_) => {
                    let pos_args = [DynDisplay::new(val)];
                    let named_args = HashMap::new();
                    let args = ParsedFormat::parse::<
                        [DynDisplay<'_>; 1],
                        HashMap<String, DynDisplay<'_>>,
                    >(
                        ext_format_str.as_ref().unwrap(), &pos_args, &named_args
                    )
                    .unwrap();
                    format!("{}", args)
                }
            },
            None => return None,
        };

        let node_lines: Vec<String> = node_str.lines().map(String::from).collect();
        if node_lines.is_empty() {
            return Some(Drawing {
                lines: vec!["".to_string()],
                width: 0,
                root_col: 0,
            });
        }

        let node_width = node_lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        // Base case: Leaf node
        if node.left.is_none() && node.right.is_none() {
            return Some(Drawing {
                lines: node_lines,
                width: node_width,
                root_col: node_width / 2,
            });
        }

        // Recursive step: Process children
        let left_drawing = node
            .left
            .as_ref()
            .and_then(|n| Self::draw_subtree(n, ext_format_str));
        let right_drawing = node
            .right
            .as_ref()
            .and_then(|n| Self::draw_subtree(n, ext_format_str));

        // Combine drawings
        match (left_drawing, right_drawing) {
            (Some(left), Some(right)) => Self::combine_both(node_lines, node_width, left, right),
            (Some(left), None) => Self::combine_single(node_lines, node_width, left),
            (None, Some(right)) => Self::combine_single(node_lines, node_width, right),
            (None, None) => Some(Drawing {
                lines: node_lines,
                width: node_width,
                root_col: node_width / 2,
            }),
        }
    }

    /// Combine node drawing with both left and right child drawings.
    fn combine_both(
        node_lines: Vec<String>,
        node_width: usize,
        left: Drawing,
        right: Drawing,
    ) -> Option<Drawing> {
        // Calculate spacing
        let gap = 1; // Minimum gap between subtrees
        let left_width = left.width;
        let right_width = right.width;

        // Calculate total width and center position
        let total_width = left_width + gap + right_width;
        let left_root_pos = left.root_col;
        let right_root_pos = left_width + gap + right.root_col;

        // Center the root node
        let root_center_pos = (left_root_pos + right_root_pos) / 2;
        let node_padding = if node_width <= total_width {
            root_center_pos - node_width / 2
        } else {
            0 // Node is wider than children, will need to expand
        };

        // Recalculate total width if node is wider
        let total_width = max(total_width, node_padding + node_width);

        // Build result from scratch
        let mut result = Vec::new();

        // Add the node text lines with proper padding
        for line in node_lines {
            let padding = " ".repeat(node_padding);
            let right_padding = " ".repeat(total_width - node_padding - line.chars().count());
            result.push(format!("{}{}{}", padding, line, right_padding));
        }

        // Create connector line
        let mut connector = String::new();
        for i in 0..total_width {
            if i == left_root_pos {
                connector.push_str(RIGHT_BOTTOM);
            } else if i == right_root_pos {
                connector.push_str(LEFT_BOTTOM);
            } else if i == root_center_pos {
                connector.push_str(TOP_T);
            } else if i > left_root_pos && i < right_root_pos {
                connector.push_str(H_LINE);
            } else {
                connector.push(' ');
            }
        }
        result.push(connector);

        // Combine child lines
        let max_child_height = max(left.lines.len(), right.lines.len());
        for i in 0..max_child_height {
            let mut line = String::new();

            // Left side
            if i < left.lines.len() {
                line.push_str(&left.lines[i]);
                line.push_str(&" ".repeat(left_width - left.lines[i].chars().count()));
            } else {
                line.push_str(&" ".repeat(left_width));
            }

            // Gap
            line.push_str(&" ".repeat(gap));

            // Right side
            if i < right.lines.len() {
                line.push_str(&right.lines[i]);
                let right_padding = total_width - line.chars().count();
                if right_padding > 0 {
                    line.push_str(&" ".repeat(right_padding));
                }
            } else {
                let remaining = total_width - line.chars().count();
                if remaining > 0 {
                    line.push_str(&" ".repeat(remaining));
                }
            }

            result.push(line);
        }

        Some(Drawing {
            lines: result,
            width: total_width,
            root_col: root_center_pos,
        })
    }

    /// Combine node drawing with a single child drawing (left or right).
    fn combine_single(
        node_lines: Vec<String>,
        node_width: usize,
        child: Drawing,
    ) -> Option<Drawing> {
        // Calculate where the vertical line should go
        let node_center = node_width / 2;
        let child_center = child.root_col;

        // Determine connector position (vertical pipe position)
        let connector_pos = child_center;

        // Determine node position to center above connector
        let node_padding = if connector_pos > node_center {
            connector_pos - node_center
        } else {
            0
        };

        // Calculate total width
        let total_width = max(node_padding + node_width, child.width);

        // Build result
        let mut result = Vec::new();

        // Add node text lines
        for line in node_lines {
            let padding = " ".repeat(node_padding);
            let right_padding = " ".repeat(total_width - node_padding - line.chars().count());
            result.push(format!("{}{}{}", padding, line, right_padding));
        }

        // Add vertical line directly (no connector character) - THIS IS THE FIXED PART
        let mut v_line = String::with_capacity(total_width);
        for i in 0..total_width {
            if i == connector_pos {
                v_line.push_str(V_LINE);
            } else {
                v_line.push(' ');
            }
        }
        result.push(v_line);

        // Add child lines
        for line in child.lines {
            let padding_needed = total_width - line.chars().count();
            let padded_line = if padding_needed > 0 {
                format!("{}{}", line, " ".repeat(padding_needed))
            } else {
                line
            };
            result.push(padded_line);
        }

        Some(Drawing {
            lines: result,
            width: total_width,
            root_col: connector_pos,
        })
    }
}

#[derive(Clone)]
struct CellDisplay {
    str: Option<String>,
}

impl CellDisplay {
    fn new() -> CellDisplay {
        CellDisplay { str: None }
    }

    fn with_content(s: String) -> CellDisplay {
        CellDisplay { str: Some(s) }
    }
}

type DisplayRows = Vec<Vec<CellDisplay>>;

// Trim unnecessary leading whitespace from all rows
fn trim_left_whitespace(rows: &mut Vec<String>) {
    if rows.is_empty() {
        return;
    }

    // Find minimum leading whitespace
    let mut min_space = rows[0].len();
    for row in rows.iter() {
        if let Some(first_non_space) = row.find(|c| c != ' ') {
            if first_non_space == 0 {
                return; // No trimming needed
            }
            min_space = min_space.min(first_non_space);
        }
    }

    // Trim each row
    for i in 0..rows.len() {
        rows[i] = rows[i][min_space..].to_string();
    }
}

impl<T> BinTree<T>
where
    T: Display + Clone,
{
    // Build tree representation using level-order traversal
    fn build_display_rows(&self, extra_val_fmt: &Option<String>) -> DisplayRows {
        let max_depth = self.get_max_depth() as usize;
        if max_depth == 0 {
            return Vec::new(); // Empty tree
        }

        // Create a vector to hold each level of the tree
        let mut rows: DisplayRows = vec![Vec::new(); max_depth];

        // Use a queue for level-order traversal (node, level, position)
        let mut queue: Vec<(Option<&BinTreeNode<T>>, usize, usize)> = Vec::new();
        queue.push((Some(&self.root), 0, 0));

        // Track the maximum position filled at each level
        let mut max_positions: Vec<usize> = vec![0; max_depth];

        while let Some((maybe_node, level, pos)) = queue.pop() {
            if level >= max_depth {
                continue;
            }

            // Ensure we have space in the row for this position
            while pos > max_positions[level] {
                rows[level].push(CellDisplay::new());
                max_positions[level] += 1;
            }

            match maybe_node {
                Some(node) => {
                    // Add this node's value
                    if let Some(val) = &node.value {
                        if extra_val_fmt.is_none() {
                            rows[level].push(CellDisplay::with_content(val.to_string()));
                        } else {
                            let pos_args = [DynDisplay::new(val)];
                            let named_args = HashMap::new();
                            let args = ParsedFormat::parse::<
                                [DynDisplay<'_>; 1],
                                HashMap<String, DynDisplay<'_>>,
                            >(
                                extra_val_fmt.as_ref().unwrap(), &pos_args, &named_args
                            )
                            .unwrap();
                            rows[level].push(CellDisplay::with_content(format!("{}", args)));
                        }
                    } else {
                        rows[level].push(CellDisplay::new());
                    }
                    max_positions[level] += 1;

                    // Queue children (right first since we're using a LIFO queue)
                    let left_pos = pos * 2;
                    let right_pos = pos * 2 + 1;

                    queue.push((
                        node.right.as_ref().map(|n| n.as_ref()),
                        level + 1,
                        right_pos,
                    ));
                    queue.push((node.left.as_ref().map(|n| n.as_ref()), level + 1, left_pos));
                }
                None => {
                    // Add a placeholder for missing nodes
                    rows[level].push(CellDisplay::new());
                    max_positions[level] += 1;
                }
            }
        }

        rows
    }

    // Format rows into strings with proper spacing and branch connections
    fn format_rows(&self, rows: &DisplayRows) -> Vec<String> {
        // Calculate cell width needed based on content
        let mut cell_width = 3; // Minimum width

        for row in rows {
            for cell in row {
                if let Some(content) = &cell.str {
                    cell_width = max(cell_width, content.len());
                }
            }
        }

        // Make cell width odd for proper centering
        if cell_width % 2 == 0 {
            cell_width += 1;
        }

        let mut formatted_rows: Vec<String> = Vec::new();
        let row_count = rows.len();

        // Process rows from bottom to top (leaf nodes first)
        let mut row_elem_count = 1 << (row_count - 1); // Max number of nodes at lowest level
        let mut left_pad = 0; // Accumulated left padding for each level

        for r in 0..row_count {
            // Get the row data (from bottom to top)
            let row_index = row_count - r - 1;
            let cd_row = &rows[row_index];

            // Calculate spacing for this level
            let space = ((1 << r) * (cell_width + 1) / 2) - 1;

            // Format the node content line
            let mut content_line = String::new();

            for c in 0..row_elem_count {
                // Add spacing before cell
                if c > 0 {
                    content_line.push_str(&" ".repeat(left_pad * 2 + 1));
                } else {
                    content_line.push_str(&" ".repeat(left_pad));
                }

                // Add the cell content
                if c < cd_row.len() {
                    if let Some(str) = &cd_row[c].str {
                        let total_padding = cell_width - str.len();
                        let left_padding = total_padding / 2;
                        let right_padding = total_padding - left_padding;

                        // Alternate padding based on even/odd column for balanced appearance
                        let (left_pad, right_pad) = if c % 2 == 0 {
                            (left_padding, right_padding)
                        } else {
                            (right_padding, left_padding)
                        };

                        content_line.push_str(&" ".repeat(left_pad));
                        content_line.push_str(str);
                        content_line.push_str(&" ".repeat(right_pad));
                    } else {
                        content_line.push_str(&" ".repeat(cell_width));
                    }
                } else {
                    content_line.push_str(&" ".repeat(cell_width));
                }
            }

            formatted_rows.push(content_line);

            // Skip branch lines for the topmost level
            if row_elem_count == 1 {
                break;
            }

            // Format branch lines with slashes
            let mut left_space = space + 1;
            let mut right_space = space - 1;

            for _ in 0..space {
                let mut branch_line = String::new();

                for c in 0..row_elem_count {
                    if c < cd_row.len() {
                        if c % 2 == 0 {
                            // Left child branch
                            branch_line.push_str(&" ".repeat(if c > 0 {
                                left_space * 2 + 1
                            } else {
                                left_space
                            }));

                            if cd_row[c].str.is_some() {
                                branch_line.push('/');
                            } else {
                                branch_line.push(' ');
                            }

                            branch_line.push_str(&" ".repeat(right_space + 1));
                        } else {
                            // Right child branch
                            branch_line.push_str(&" ".repeat(right_space));

                            if cd_row[c].str.is_some() {
                                branch_line.push('\\');
                            } else {
                                branch_line.push(' ');
                            }
                        }
                    } else {
                        // Empty space for non-existent nodes
                        branch_line.push_str(&" ".repeat(left_space + right_space + 1));
                    }
                }

                formatted_rows.push(branch_line);
                left_space += 1;
                if right_space > 0 {
                    right_space -= 1;
                }
            }

            // Adjust for next level
            left_pad += space + 1;
            row_elem_count /= 2;
        }

        // Reverse to get top-down ordering
        formatted_rows.reverse();
        formatted_rows
    }

    pub fn dump(&self, fmt_val_arg: Option<String>) -> String {
        if self.get_max_depth() == 0 {
            return "<empty tree>".to_string();
        }

        // Get display representation and format it
        let rows = self.build_display_rows(&fmt_val_arg);
        let mut formatted_rows = self.format_rows(&rows);

        // Trim unnecessary left whitespace
        trim_left_whitespace(&mut formatted_rows);

        formatted_rows.join("\n")
    }
}

impl<T: Display + Clone> Display for BinTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            if f.precision().is_none() {
                write!(f, "{}", self.dump(None))
            } else {
                write!(
                    f,
                    "{}",
                    self.dump(Some(format!("{{:.{}}}", f.precision().unwrap())))
                )
            }
        } else {
            if f.precision().is_none() {
                write!(f, "{}", self.format_tree(None))
            } else {
                write!(
                    f,
                    "{}",
                    self.format_tree(Some(format!("{{:.{}}}", f.precision().unwrap())))
                )
            }
        }
    }
}
