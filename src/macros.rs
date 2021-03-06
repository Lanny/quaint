/// Convert given set of tuples into `Values`.
///
/// ```rust
/// # use quaint::{col, values, ast::*, visitor::{Visitor, Sqlite}};
/// # fn main() -> Result<(), quaint::error::Error> {
///
/// let condition = Row::from((col!("id"), col!("name")))
///     .in_selection(values!((1, "Musti"), (2, "Naukio")));
///
/// let query = Select::from_table("cats").so_that(condition);
/// let (sql, _) = Sqlite::build(query)?;
///
/// assert_eq!(
///     "SELECT `cats`.* FROM `cats` WHERE (`id`,`name`) IN (VALUES (?,?),(?,?))",
///     sql
/// );
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! values {
    ($($x:expr),*) => (
        Values::from(std::iter::empty() $(.chain(std::iter::once(Row::from($x))))*)
    );
}

/// Marks a given string or a tuple as a column. Useful when using a column in
/// calculations, e.g.
///
/// ``` rust
/// # use quaint::{col, val, ast::*, visitor::{Visitor, Sqlite}};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let join = "dogs".on(("dogs", "slave_id").equals(Column::from(("cats", "master_id"))));
///
/// let query = Select::from_table("cats")
///     .value(Table::from("cats").asterisk())
///     .value(col!("dogs", "age") - val!(4))
///     .inner_join(join);
///
/// let (sql, params) = Sqlite::build(query)?;
///
/// assert_eq!(
///     "SELECT `cats`.*, (`dogs`.`age` - ?) FROM `cats` INNER JOIN `dogs` ON `dogs`.`slave_id` = `cats`.`master_id`",
///     sql
/// );
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! col {
    ($e1:expr) => {
        Expression::from(Column::from($e1))
    };

    ($e1:expr, $e2:expr) => {
        Expression::from(Column::from(($e1, $e2)))
    };
}

/// Marks a given string as a value. Useful when using a value in calculations,
/// e.g.
///
/// ``` rust
/// # use quaint::{col, val, ast::*, visitor::{Visitor, Sqlite}};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let join = "dogs".on(("dogs", "slave_id").equals(Column::from(("cats", "master_id"))));
///
/// let query = Select::from_table("cats")
///     .value(Table::from("cats").asterisk())
///     .value(col!("dogs", "age") - val!(4))
///     .inner_join(join);
///
/// let (sql, params) = Sqlite::build(query)?;
///
/// assert_eq!(
///     "SELECT `cats`.*, (`dogs`.`age` - ?) FROM `cats` INNER JOIN `dogs` ON `dogs`.`slave_id` = `cats`.`master_id`",
///     sql
/// );
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! val {
    ($val:expr) => {
        Expression::from($val)
    };
}

macro_rules! value {
    ($target:ident: $kind:ty,$paramkind:ident,$that:expr) => {
        impl<'a> From<$kind> for crate::ast::Value<'a> {
            fn from(that: $kind) -> Self {
                let $target = that;
                crate::ast::Value::$paramkind(Some($that))
            }
        }

        impl<'a> From<Option<$kind>> for crate::ast::Value<'a> {
            fn from(that: Option<$kind>) -> Self {
                match that {
                    Some(val) => crate::ast::Value::from(val),
                    None => crate::ast::Value::$paramkind(None),
                }
            }
        }
    };
}

macro_rules! aliasable {
    ($($kind:ty),*) => (
        $(
            impl<'a> Aliasable<'a> for $kind {
                type Target = Table<'a>;

                fn alias<T>(self, alias: T) -> Self::Target
                where
                    T: Into<Cow<'a, str>>,
                {
                    let table: Table = self.into();
                    table.alias(alias)
                }
            }
        )*
    );
}

macro_rules! function {
    ($($kind:ident),*) => (
        $(
            impl<'a> From<$kind<'a>> for Function<'a> {
                fn from(f: $kind<'a>) -> Self {
                    Function {
                        typ_: FunctionType::$kind(f),
                        alias: None,
                    }
                }
            }

            impl<'a> From<$kind<'a>> for Expression<'a> {
                fn from(f: $kind<'a>) -> Self {
                    Function::from(f).into()
                }
            }
        )*
    );
}

macro_rules! expression {
    ($kind:ident,$paramkind:ident) => {
        impl<'a> From<$kind<'a>> for Expression<'a> {
            fn from(that: $kind<'a>) -> Self {
                Expression {
                    kind: ExpressionKind::$paramkind(that),
                    alias: None,
                }
            }
        }
    };
}
