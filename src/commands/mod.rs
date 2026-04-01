macro_rules! мяу_предмет {
    ($item:item) => { $item };
}

мяу_предмет! { pub mod moderation; }
мяу_предмет! { pub mod terminal; }
мяу_предмет! { pub mod server; }
мяу_предмет! { pub mod utility; }

мяу_предмет! { pub mod settings; }
