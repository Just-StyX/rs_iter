# Custom Team Iterator Framework in Rust

A robust implementation of the idiomatic Rust iterator pattern for a custom data collection using Copy-on-Write (`std::borrow::Cow`) smart pointers. 

This framework demonstrates how to seamlessly achieve memory safety, zero-cost abstractions, and flexible data access patterns across Rust's ownership boundaries by implementing the "Holy Trinity" of collection iteration.

---

## 🚀 The Three Iteration Categories

Rust collections typically expose three variants of iteration based on how memory is handled. Choosing the right one optimizes execution speed and memory management:

### 1. Iterating by Reference (`&Team`) — *Read-Only*
* **Why:** You need to inspect, print, filter, or process data without altering it and without taking ownership of the `Team`.
* **When:** Ideal for rendering rosters, searching for specific players, calculating statistics, or broadcasting state changes.
* **How:** Yields immutable references (`&Cow<'a, str>`). It uses decoupled lifetimes (`'b` for the loop borrow, `'a` for the underlying data) so multiple reference loops can be safely run sequentially.

### 2. Iterating by Mutable Reference (`&mut Team`) — *In-Place Modification*
* **Why:** You need to modify data values directly inside the collection without tearing down or reconstructing the underlying `Vec`.
* **When:** Perfect for processing name changes, updating ranks, or appending statuses (e.g., adding `" (Senior)"` to a player's profile).
* **How:** Yields mutable references (`&mut Cow<'a, str>`). Leverages `Cow::to_mut()` to ensure string allocations are only modified inline, automatically converting borrowed references to owned allocations on-demand.

### 3. Iterating by Value (`Team`) — *Consuming / Ownership Transfer*
* **Why:** The `Team` struct has completed its lifecycle, and you want to pass full ownership of its individual elements to another component without allocating copies.
* **When:** Used when transforming data structures (e.g., mapping a `Team` into a database entity model) or threading elements into a new async task context.
* **How:** Yields owned elements (`Cow<'a, str>`). This iteration explicitly consumes the `Team` instance, moving its memory and rendering the original variable inaccessible afterwards to prevent use-after-move compiler bugs.

---

## 🛠 Features Included

* **`DoubleEndedIterator` Support:** Every category fully supports reversible iteration via `.rev()`. Elements can be efficiently extracted from both the front and the back of the collection.
* **Borrow Checker Isolation:** Decoupled short-lived transient loop references from underlying persistent struct data strings to eradicate accidental long-lived mutable locking bugs.

---

## 📖 Complete Framework Implementation

Below is the single-file source code implementing the `Team` structure, custom iterators, and code pipeline verification:

```rust
use std::{borrow::Cow, slice::{Iter, IterMut}, vec::IntoIter};

fn main() {
    // Instantiate our roster using From conversion traits
    let mut roaster: Team = ("Rustaceans", vec!["Alice", "Bob", "Catline"]).into();
    
    // --------------------------------------------------------------------------
    // CATEGORY 2 Demo: Mutable Modification
    // --------------------------------------------------------------------------
    for player in &mut roaster { 
        // .to_mut() automatically transitions Cow::Borrowed to Cow::Owned if mutated
        player.to_mut().push_str(" (Senior)");
    }
    
    // --------------------------------------------------------------------------
    // CATEGORY 1 Demo: Immutable Forward Reading
    // --------------------------------------------------------------------------
    println!("--- {} Roaster (FORWARD) ---", roaster.name.to_ascii_uppercase());
    for player in &roaster { 
        println!("Player: {}", player);
    }

    // --------------------------------------------------------------------------
    // DoubleEndedIterator Demo: Reversing Iteration
    // --------------------------------------------------------------------------
    println!("\n--- {} Roaster (REVERSED) ---", roaster.name.to_ascii_uppercase());
    for player in (&roaster).into_iter().rev() { 
        println!("Player: {}", player);
    }
}

/// A team structure holding names wrapped in standard copy-on-write smart pointers.
pub struct Team<'a> {
    pub name: Cow<'a, str>,
    pub players: Vec<Cow<'a, str>>,
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
pub struct TeamIter<'b, 'a> {
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
        self.inner.next_back()
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
pub struct TeamIterMut<'b, 'a> {
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
pub struct TeamIntoIter<'a> {
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
```

---

## 🧪 Ownership Verification Tests

The correctness of Category 3 (`TeamIntoIter`) ownership moving characteristics can be mathematically and statically proved via the unit test pipeline below:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_iter_ownership_transfer() {
        let roaster: Team = ("Rustaceans", vec!["Alice", "Bob"]).into();

        let mut players_collected = Vec::new();
        // Consuming execution taking complete ownership of the Team struct
        for player in roaster { 
            players_collected.push(player);
        }

        assert_eq!(players_collected.len(), 2);
        assert_eq!(players_collected[0], "Alice");
        assert_eq!(players_collected[1], "Bob");

        // The assertion below is enforced statically by the compiler.
        // Uncommenting it will trigger E0382 (Use of moved value) and stop compilation:
        // println!("{}", roaster.name); 
    }
}
```
