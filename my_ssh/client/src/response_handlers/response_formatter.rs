use colored::*;
use std::io::Write;
/// A response formatter that renders structured server responses with appropriate styling.
/// 
/// `ShowResponse` processes formatted text responses from the secure shell server and
/// displays them with proper styling, colors, and layout. It handles various response
/// types including error messages, directory listings, search results, and normal output.
/// 
/// # Response Format System
/// 
/// The formatter processes responses that contain formatting control sequences:
/// - **Format Markers**: `?&X` where X indicates the format type
/// - **Color Codes**: `^!`, `^#`, `^@` for different colors
/// - **Color Stops**: `~~` to end color formatting
/// - **Section Separators**: `\n\n` to separate content blocks
/// 
/// # Supported Format Types
/// 
/// | Marker | Type | Description | Styling |
/// |--------|------|-------------|---------|
/// | `?&E` | Error | Error messages | Plain red text |
/// | `?&L` | List | Directory listings | Colored columns |
/// | `?&C` | Colored | Search results | Highlighted matches |
/// | `?&N` | Normal | Standard output | Plain text |
/// 
/// # Color System
/// 
/// | Code | Color | Usage |
/// |------|-------|-------|
/// | `^!` | Blue | Directories, special files |
/// | `^#` | Green | Regular files |
/// | `^@` | Red | Search matches, errors |
/// | `~~` | Reset | End color formatting |
/// 
/// # Examples
/// 
/// ## Basic Usage
/// ```rust
/// // Display server response
/// let response = ShowResponse::new("?&NHello World\n\n".to_string());
/// response.show();
/// // Output: Hello World
/// ```
/// 
/// ## Directory Listing
/// ```rust
/// let listing = "?&L^!documents\n\n^#file.txt\n\n^!pictures\n\n";
/// let response = ShowResponse::new(listing.to_string());
/// response.show();
/// // Output: documents  file.txt   pictures  (in columns with colors)
/// ```
/// 
/// ## Search Results
/// ```rust
/// let search = "?&CThis is a ^@match~~ in text\n\n";
/// let response = ShowResponse::new(search.to_string());
/// response.show();
/// // Output: This is a match in text (with "match" highlighted in red)
/// ```
pub struct ShowResponse {
    response: String,
}
/// Maps color code strings to their corresponding `colored::Color` values.
/// 
/// This function translates the custom color codes used in the shell response
/// format to the standard color enumeration used by the `colored` crate.
/// 
/// # Parameters
/// 
/// - `color`: Color code string (e.g., "^!", "^#", "^@")
/// 
/// # Returns
/// 
/// The corresponding `colored::Color` value for styling text.
/// 
/// # Color Mappings
/// 
/// | Input Code | Output Color | Typical Usage |
/// |------------|--------------|---------------|
/// | `"^!"` | `Color::Blue` | Directories, folders |
/// | `"^#"` | `Color::Green` | Regular files |
/// | `"^@"` | `Color::Red` | Matches, highlights |
/// | *other* | `Color::White` | Default/fallback |
/// 
/// # Examples
/// 
/// ```rust
/// assert_eq!(get_color("^!"), Color::Blue);
/// assert_eq!(get_color("^#"), Color::Green);
/// assert_eq!(get_color("unknown"), Color::White);
/// ```
fn get_color(color: &str) -> colored::Color {
    match color {
        "^!" => Color::Blue,
        "^#" => Color::Green,
        _ => Color::White,
    }
}

impl ShowResponse {
    pub fn new(response: String) -> Self {
        ShowResponse { response }
    }
       /// Displays content in a columnar list format with color coding.
    /// 
    /// This method renders directory listings and similar structured content
    /// in a three-column layout with proper spacing and color formatting.
    /// Each item can optionally include color codes for different file types.
    /// 
    /// # Parameters
    /// 
    /// - `word`: Vector of strings representing items to display
    /// 
    /// # Layout Behavior
    /// 
    /// - **Columns**: Items are arranged in 3 columns
    /// - **Spacing**: Automatic padding based on longest item
    /// - **Wrapping**: New line after every 3 items
    /// - **Colors**: Processes color codes for individual items
    /// 
    /// # Color Processing
    /// 
    /// Items can include color codes:
    /// - `^!filename` - Blue colored filename (directories)
    /// - `^#filename` - Green colored filename (files)
    /// - `filename` - White/default colored filename
    /// 
    fn show_list_style(&self, word: Vec<&str>) {
        fn print_space(len: usize) {
            for _ in 0..len {
                print!(" ");
            }
        }
        let mut count = 0;
        let mut max_len = 0usize;
        for e in &word {
            max_len = std::cmp::max(max_len, e.chars().count());
        }
        for e in word {
            if e.starts_with('^') {
                let color_code = &e[0..2];
                print!("{}", e[2..].color(get_color(color_code)));
                print_space(max_len + 1 - e[2..].len());
            } else {
                print!("{}", e);
                print_space(max_len + 1 - e.len());
            }
            count += 1;
            if count % 3 == 0 {
                count = 0;
                println!()
            }
            std::io::stdout().flush().unwrap();
        }
        println!()
    }
    // Displays content with inline color highlighting for search matches.
    /// 
    /// This method renders text with embedded color markers, typically used
    /// for displaying search results where matches are highlighted in red.
    /// It processes color start (`^@`) and stop (`~~`) markers within the text.
    /// 
    /// # Parameters
    /// 
    /// - `word`: Vector of strings containing text with color markers
    /// 
    /// # Color Marker Processing
    /// 
    /// - **`^@`**: Start red highlighting (bright red color)
    /// - **`~~`**: Stop red highlighting (return to normal color)
    /// - Characters between markers are displayed in bright red
    /// - Markers themselves are not displayed
    /// 
    /// # Character-by-Character Processing
    /// 
    /// The method processes each character individually to handle:
    /// - Overlapping color regions
    /// - Multiple highlighted sections
    /// - Proper color state management
    /// - Marker sequence detection
    fn show_grep_style(&self, word: Vec<&str>) {
        for w in word {
            if !w.is_empty() {
                let mut red_status = false;
                let chars: Vec<_> = w.chars().collect();
                let mut i = 0;
                while i < chars.len() {
                    let c = chars[i];
                    if i + 1 < chars.len() && c == '^' && chars[i + 1] == '@' {
                        red_status = true;
                        i += 2;
                        continue;
                    }
                    if i + 1 < chars.len() && c == '~' && chars[i + 1] == '~' {
                        red_status = false;
                        i += 2;
                        continue;
                    }
                    if red_status {
                        print!("{}", c.to_string().bright_red());
                    } else {
                        print!("{}", c);
                    }
                    i += 1;
                }
            }
        }
        println!();
    }
     /// Parses the response string and routes content to appropriate display methods.
    /// 
    /// This method serves as the main response processor, analyzing format markers
    /// and delegating to specialized display methods based on content type. It
    /// handles the parsing of the structured response format used by the shell server.
    /// 
    /// # Response Format Structure
    /// 
    /// Responses follow the pattern:
    /// ```text
    /// ?&[TYPE][CONTENT]\n\n?&[TYPE][CONTENT]\n\n...
    /// ```
    /// 
    /// Where:
    /// - `?&`: Format marker prefix
    /// - `[TYPE]`: Single character indicating format type
    /// - `[CONTENT]`: Content to be formatted
    /// - `\n\n`: Section separator
    /// 
    /// # Processing Flow
    /// 
    /// 1. **Split on Markers**: Divides response on `?&` boundaries
    /// 2. **Filter Empty**: Removes empty sections
    /// 3. **Extract Type**: Reads first character as format type
    /// 4. **Extract Content**: Splits content on `\n\n` separators
    /// 5. **Route Display**: Calls appropriate display method
    /// 
    /// # Format Type Handling
    /// 
    /// | Type | Handler | Description |
    /// |------|---------|-------------|
    /// | `E` | Direct print | Error messages (plain text) |
    /// | `L` | `show_list_style()` | Directory listings (columnar) |
    /// | `C` | `show_grep_style()` | Colored text (inline highlights) |
    /// | `N` | Space-separated | Normal output (plain text) |
    /// | *other* | Ignored | Unknown format types |
    /// | *none* | Debug message | Empty or malformed sections |
    /// 
    fn split_response(&self) {
        let props: Vec<&str> = self
            .response
            .split("?&")
            .filter(|f| !f.is_empty())
            .collect();
        for w in props {
            let word: Vec<&str> = w[1..].split("\n\n").filter(|f| !f.is_empty()).collect();
            //dbg!(&word);
            match w.chars().next() {
                Some('E') => {
                    println!("{}", word[0]);
                }
                Some('L') => {
                    self.show_list_style(word);
                }
                Some('C') => {
                    self.show_grep_style(word);
                }
                Some('N') => {
                    for w in word {
                        if !w.is_empty() {
                            print!("{} ", w);
                        }
                    }
                    println!();
                }
                Some(_) => (),
                None => {
                    println!("String gol");
                }
            }
        }
    }
    pub fn show(&self) {
        self.split_response();
    }
}
