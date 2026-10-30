use std::{borrow::Cow, slice::{Iter, IterMut}, vec::IntoIter};

fn main() {
    let mut roaster: Team = ("Rustaceans", vec!["Alice", "Bob", "Catline"]).into();
    
    // Use `&mut roaster` to call the mutable IntoIterator implementation
    for player in &mut roaster { 
        // Fix 2: Use `.to_mut()` to gain mutable access to the underlying String inside the Cow
        player.to_mut().push_str(" (Senior)");
    }
    
    println!("--- {} Roaster ---", roaster.name.to_ascii_uppercase());
    // This calls the IntoIterator implementation
    for player in &roaster { 
        println!("Player: {}", player);
    }

    println!("\n--- {} Roaster (REVERSED) ---", roaster.name.to_ascii_uppercase());
    for player in (&roaster).into_iter().rev() { 
        println!("Player: {}", player);
    }
}

struct Team<'a> {
    name: Cow<'a, str>,
    players: Vec<Cow<'a, str>>,
}

impl<'a> From<(&'a str, Vec<&'a str>)> for Team<'a> {
    fn from(value: (&'a str, Vec<&'a str>)) -> Self {
        Self {
            name: Cow::Borrowed(value.0),
            players: value.1.into_iter().map(|s| Cow::Borrowed(s)).collect()
        }
    }
}
// ==========================================================================
// CATEGORY 1: Iterating by Reference (&Team) -> Yields &Cow<'a, str>
// ==========================================================================
struct TeamIter<'b, 'a> {
    inner: Iter<'b, Cow<'a, str>>
}

impl<'b, 'a> Iterator for TeamIter<'b, 'a> {
    type Item = &'b Cow<'a, str>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'b, 'a> DoubleEndedIterator for TeamIter<'b, 'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back() // Calls standard slice iterator's next_back
    }
}

impl<'a> IntoIterator for &'a Team<'a> {
    type Item = &'a Cow<'a, str>;
    type IntoIter = TeamIter<'a, 'a>;
    fn into_iter(self) -> Self::IntoIter {
        TeamIter { inner: self.players.iter() }
    }
}

// ===================================================================================
// CATEGORY 2: Iterating by Mutable Reference (&mut Team) -> Yields &mut Cow<'a, str>
// ===================================================================================
struct TeamIterMut<'b, 'a> {
    inner: IterMut<'b, Cow<'a, str>>
}

impl<'b, 'a> Iterator for TeamIterMut<'b, 'a> {
    type Item = &'b mut Cow<'a, str>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'b, 'a> DoubleEndedIterator for TeamIterMut<'b, 'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'b, 'a> IntoIterator for &'b mut Team<'a> {
    type Item = &'b mut Cow<'a, str>;
    type IntoIter = TeamIterMut<'b, 'a>;
    fn into_iter(self) -> Self::IntoIter {
        TeamIterMut { inner: self.players.iter_mut() }
    }
}

// ===================================================================================
// CATEGORY 3: Iterating by Value (Team) -> Yields Cow<'a, str>
// ===================================================================================
struct TeamIntoIter<'a> {
    inner: IntoIter<Cow<'a, str>>
}

impl<'a> Iterator for TeamIntoIter<'a> {
    type Item = Cow<'a, str>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> DoubleEndedIterator for TeamIntoIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'a> IntoIterator for Team<'a> {
    type Item = Cow<'a, str>;
    type IntoIter = TeamIntoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        TeamIntoIter { inner: self.players.into_iter() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_iter_ownership_transfer() {
        let roaster: Team = ("Rustaceans", vec!["Alice", "Bob"]).into();

        // Collect elements using the consuming into_iter execution
        let mut players_collected = Vec::new();
        for player in roaster { // <--- roaster ownership is MOVED here
            players_collected.push(player);
        }

        // Assert that the data was successfully transferred
        assert_eq!(players_collected.len(), 2);
        assert_eq!(players_collected[0], "Alice");
        assert_eq!(players_collected[1], "Bob");

        // UNCOMMENTING THE LINE BELOW WILL TRIPPLE A COMPILER ERROR:
        // println!("{}", roaster.name); 
        // ^ Error: use of moved value: `roaster`
    }
}

/// Verify that trying to use `roaster` after an `into_iter` loop causes a compile error.
/// 
/// ```compile_fail
/// use your_crate_name::Team; // Replace with your actual crate name
/// 
/// let roaster: Team = ("Rustaceans", vec!["Alice", "Bob"]).into();
/// for player in roaster {} 
/// 
/// // This line will fail compilation exactly as intended!
/// println!("{}", roaster.name); 
/// ```
fn _test_compile_failure_documentation() {}