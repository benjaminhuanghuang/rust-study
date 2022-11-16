# std::fmt::Display
The macros println!, print!, write!, writeln!, and format! all rely on the Display and Debug traits, and these rely on trait implementations provided by programmers to convert from {} to what is printed to the console.

Debug relies on the "{:?}" colon and question mark syntax.

Display requires that types implement a fmt method, which returns fmt::Result


```
impl Display for FileState {
  fn fmt(&self, f:
         &mut fmt::Formatter,
  ) -> fmt::Result {                      
    match *self {
      FileState::Open => write!(f, "OPEN"),
      FileState::Closed => write!(f, "CLOSED"),
    }
  }
}
 
impl Display for File {
   fn fmt(&self, f:
          &mut fmt::Formatter,
   ) -> fmt::Result {                     
      write!(f, "<{} ({})>",
             self.name, self.state)       
   }
}

```