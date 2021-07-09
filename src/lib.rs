use std::fmt::Write; // Uses write!

/// Helper function to generate the phrase of how many bottles there
/// are.  There are three basic cases:
/// - \# bottles
/// - 1 bottle
/// - No more bottles
/// 
/// # Arguments
/// * `n` The number to use in the phrase
/// * `capitalize` If the first letter should be capitalized
/// # Returns
/// A string representing the phrase
fn phrase(n: u32, capitalize: bool) -> String {
    let mut phrase = String::new();
    match n {
        0 => {
            if capitalize {
                write!(phrase, "No more bottles").unwrap()
            } else {
                write!(phrase, "no more bottles").unwrap()
            }
        },
        1 => write!(phrase, "1 bottle").unwrap(),
        _ => write!(phrase, "{} bottles", n).unwrap(),
    };
    phrase
}

/// Helper function to generate the action to be taken
/// are.  There are three cases
/// - Take one down and pass it around
/// - Take it down and pass it around
/// - Go to the store and by some more
/// 
/// # Arguments
/// * `n` The number of bottles remaining
/// # Returns
/// A string representing the action
fn action(n: u32) -> String {
    match n {
        0 => String::from("Go to the store and buy some more,"),
        1 => String::from("Take it down and pass it around,"),
        _ => String::from("Take one down and pass it around,"),
    }
}

/// Function to generate one verse of the beer song
/// 
/// # Arguments
/// * `n` The number of bottles remaining
/// # Returns
/// A string representing the verse
pub fn verse(n: u32) -> String {
    let mut verse: String = String::new();
    let next_n = match n {
        0 => 99,
        _ => n - 1
    };
    write!(verse, "{} of beer on the wall, {} of beer.\n", phrase(n, true), phrase(n, false)).unwrap();
    write!(verse, "{} {} of beer on the wall.\n", action(n), phrase(next_n, false)).unwrap();
    verse
}

/// Function to generate a song based on a start and end number of bottles.
/// The start can be a lower number than the end.  If this happens, the song
/// will continue after the buy some more verse and keep going.  Since the
/// purchase number is 99, if the end number is larger than that, the song
/// will end after the 0 bottles verse.  If the start is larger than 99, the
/// song will allow singing from that start number.
/// 
/// # Arguments
/// * `start` The number of bottles at the start
/// * `end` The number of bottles at the start of the last verse
/// # Returns
/// A string representing the verse
pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    if start > end {
        // Normal song, start high and count down
        for n in (end..start+1).rev() {
            write!(song, "{}\n", verse(n)).unwrap();
        }
    } else {
        // Count down, go back up, and count down some more
        // Only wraps if the end is less than 99.
        for n in (0..start+1).rev() {
            write!(song, "{}\n", verse(n)).unwrap();
        }
        for n in (end..100).rev() {
            write!(song, "{}\n", verse(n)).unwrap();
        }
    }
    song.pop(); // Remove extra newline
    // println!("{}", song); // From debugging to see output
    song
}
