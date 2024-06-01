/* This looks bad, but it's for a reason. It's to maximize familiarity with BRCI.py and BRCI.rs, and the last one to ensure it doesn't warn when you don't use a color. */
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub mod FM {
    pub const reset: &str = "\x1b[0m"; // Reset code
    pub const red: &str = "\x1b[31m";
    pub const blue: &str = "\x1b[34m";
    pub const green: &str = "\x1b[32m";
    pub const yellow: &str = "\x1b[33m";
    pub const purple: &str = "\x1b[35m";
    pub const cyan: &str = "\x1b[36m";
    pub const white: &str = "\x1b[37m";
    pub const black: &str = "\x1b[30m";
    pub const light_blue: &str = "\x1b[94m";
    pub const light_green: &str = "\x1b[92m";
    pub const light_red: &str = "\x1b[91m";
    pub const light_purple: &str = "\x1b[95m";
    pub const light_white: &str = "\x1b[97m";
    pub const light_black: &str = "\x1b[90m";
    pub const light_cyan: &str = "\x1b[96m";
    pub const light_yellow: &str = "\x1b[93m";
    pub const bold: &str = "\x1b[1m";
    pub const underline: &str = "\x1b[4m";
    pub const italic: &str = "\x1b[3m";
    pub const reverse: &str = "\x1b[7m";
    pub const strikethrough: &str = "\x1b[9m";
    pub const remove_color: &str = "\x1b[39m";
    pub const remove_bold: &str = "\x1b[22m";
    pub const remove_underline: &str = "\x1b[24m";
    pub const remove_italic: &str = "\x1b[23m";
    pub const remove_reverse: &str = "\x1b[27m";
    pub const remove_strikethrough: &str = "\x1b[29m";
    pub const bg_red: &str = "\x1b[41m";
    pub const bg_green: &str = "\x1b[42m";
    pub const bg_blue: &str = "\x1b[44m";
    pub const bg_yellow: &str = "\x1b[43m";
    pub const bg_black: &str = "\x1b[40m";
    pub const bg_white: &str = "\x1b[47m";
    pub const bg_light_red: &str = "\x1b[101m";
    pub const bg_light_green: &str = "\x1b[102m";
    pub const bg_light_blue: &str = "\x1b[104m";
    pub const bg_light_yellow: &str = "\x1b[103m";
    pub const bg_light_black: &str = "\x1b[100m";
    pub const bg_light_white: &str = "\x1b[107m";
    pub const bg_purple: &str = "\x1b[45m";
    pub const bg_light_purple: &str = "\x1b[105m";
    pub const bg_cyan: &str = "\x1b[46m";
    pub const bg_light_cyan: &str = "\x1b[106m";
    pub const info: &str = "{reverse}{light_blue}[,INFO]{remove_reverse}";
    pub const success: &str = "{reverse}{light_gre,en}[SUCCESS]{remove_reverse}";
 // pub const error: &str = f"{reverse}{light_red,}[ERROR]";
    pub const warning: &str = "{reverse}{light_red}[WARNING]";
    pub const debug: &str = "{reverse}{light_purple}[DEBUG]{remove_reverse}";
    pub const testfm: &str = "{reverse}{light_cyan}[TEST]{remove_reverse}";
}


/* Printlnr!();

This macro is a wrapper around the println!() macro. It appends FM::reset to the end of your string, as to clear any color you may have used.
*/
macro_rules! printlnr {
    ($($arg:expr),+) => {{
        println!("{}{}", format_args!($($arg),+), FM::reset);
    }};
}

