use std::io::Result as IOResult;
use std::io::{self, Write};

// Animation from:https://jsfiddle.net/Diggsey/3pdgh52r/
const CRAB_ART: [&str; 68] = [
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   ¬   _'
  \ '-----' /
"#,
    r#"          
    _~^~^~_
\) /  o o  \ (/
 '-,   -  _'\
  | '----' 
"#,
    r#"          
    .~'^'^-, (/
\) /  o O  |'
 '-,   -  _'\
  | '----' 
"#,
    r#"          
    .~'^'^-, (/
\) /  o O  |'
 '-,   -  _'\
  | '----' 
"#,
    r#"          
    _~^~^~_
\) /  o o  \ (/
 '-,   -  _'\
  | '----' 
"#,
    r#"
     _~^~^~_
 \) /  o o  \ (/
   '_   ¬   _'
   / '-----' \
"#,
    r#"
      _~^~^~_
  \) /  o o  \ (/
    '_   ¬   _'
    | '-----' |
"#,
    r#"
       _~^~^~_
   \) /  o o  \ (/
     '_   ¬   _'
     \ '-----' /
"#,
    r#"
        _~^~^~_
    \) /  o o  \ (/
      '_   ¬   _'
      | '-----' |
"#,
    r#"
         _~^~^~_
     \) /  o o  \ (/
       '_   ¬   _'
       / '-----' \
"#,
    r#"
          _~^~^~_
      \) /  o o  \ (/
        '_   ¬   _'
        | '-----' |
"#,
    r#"
           _~^~^~_
       \) /  o o  \ (/
         '_   ¬   _'
         \ '-----' /
"#,
    r#"
            _~^~^~_
        \) /  o o  \ (/
          '_   ¬   _'
          | '-----' |
"#,
    r#"
             _~^~^~_
         \) /  o o  \ (/
           '_   ¬   _'
           / '-----' \
"#,
    r#"
              _~^~^~_
          \) /  o o  \ (/
            '_   ¬   _'
            | '-----' |
"#,
    r#"
               _~^~^~_
           \) /  o o  \ (/
             '_   ¬   _'
             \ '-----' /
"#,
    r#"
                _~^~^~_
            \) /  o o  \ (/
              '_   ¬   _'
              | '-----' |
"#,
    r#"
                 _~^~^~_
             \) /  o o  \ (/
               '_   ¬   _'
               / '-----' \
"#,
    r#"
                  _~^~^~_
              \) /  o o  \ (/
                '_   ¬   _'
                | '-----' |
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   ¬   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \/ /  o o  \ \/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \/ /  o o  \ \/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \/ /  o o  \ \/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   ¬   _'
                 \ '-----' /
"#,
    r#"
                   _~^~^~_
               \) /  o o  \ (/
                 '_   ¬   _'
                 \ '-----' /
"#,
    r#"          
                   _~^~^~_
               \) /  o o  \ (/
                 /'_  -   ,-'
                    '----' |
"#,
    r#"          
               \) ,-^'^'~.
                 '|  O o  \ (/
                 /'_  -   ,-'
                    '----' |
"#,
    r#"          
               \) ,-^'^'~.
                 '|  O o  \ (/
                 /'_  -   ,-'
                    '----' |
"#,
    r#"          
                   _~^~^~_
               \) /  o o  \ (/
                 /'_  -   ,-'
                    '----' |
"#,
    r#"
                  _~^~^~_
              \) /  o o  \ (/
                '_   ¬   _'
                / '-----' \
"#,
    r#"
                 _~^~^~_
             \) /  o o  \ (/
               '_   ¬   _'
               | '-----' |
"#,
    r#"
                _~^~^~_
            \) /  o o  \ (/
              '_   ¬   _'
              \ '-----' /
"#,
    r#"
               _~^~^~_
           \) /  o o  \ (/
             '_   ¬   _'
             | '-----' |
"#,
    r#"
              _~^~^~_
          \) /  o o  \ (/
            '_   ¬   _'
            / '-----' \
"#,
    r#"
             _~^~^~_
         \) /  o o  \ (/
           '_   ¬   _'
           | '-----' |
"#,
    r#"
            _~^~^~_
        \) /  o o  \ (/
          '_   ¬   _'
          \ '-----' /
"#,
    r#"
           _~^~^~_
       \) /  o o  \ (/
         '_   ¬   _'
         | '-----' |
"#,
    r#"
          _~^~^~_
      \) /  o o  \ (/
        '_   ¬   _'
        / '-----' \
"#,
    r#"
         _~^~^~_
     \) /  o o  \ (/
       '_   ¬   _'
       | '-----' |
"#,
    r#"
        _~^~^~_
    \) /  o o  \ (/
      '_   ¬   _'
      \ '-----' /
"#,
    r#"
       _~^~^~_
   \) /  o o  \ (/
     '_   ¬   _'
     | '-----' |
"#,
    r#"
      _~^~^~_
  \) /  o o  \ (/
    '_   ¬   _'
    / '-----' \
"#,
    r#"
     _~^~^~_
 \) /  o o  \ (/
   '_   ¬   _'
   | '-----' |
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   ¬   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\/ /  o o  \ \/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\/ /  o o  \ \/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\/ /  o o  \ \/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   u   _'
  \ '-----' /
"#,
    r#"
    _~^~^~_
\) /  o o  \ (/
  '_   ¬   _'
  \ '-----' /
"#,
];

fn clear_screen() -> IOResult<()> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()
}

fn animation_cycle() -> IOResult<()> {
    for crab in &CRAB_ART {
        clear_screen()?;
        println!("{}", crab);
        io::stdout().flush()?;
        std::thread::sleep(std::time::Duration::from_millis(90));
    }
    Ok(())
}

pub fn dance_crab_dance(cycles: usize) -> IOResult<()> {
    for _ in 0..cycles {
        animation_cycle()?;
    }
    Ok(())
}
