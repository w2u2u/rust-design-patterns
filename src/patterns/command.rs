// As performance, there is always a trade-off between performance and code simplicity and organisation.
// Static dispatch gives faster performance, while dynamic dispatch provides flexibility when we structure our application

/// If command is a whole struct with a bunch of functions and variables defined as separated module
/// then using this would be more suitable
///
/// # Example
/// ```
/// use design_patterns::patterns::command::trait_object::{Schema, CreateTable, AddField};
///
/// let mut schema = Schema::default();
/// let cmd = Box::new(CreateTable);
///
/// schema.add_migration(cmd);
///
/// let cmd = Box::new(AddField);
///
/// schema.add_migration(cmd);
///
/// assert_eq!(vec!["create table", "add field"], schema.execute());
/// assert_eq!(vec!["remove field", "drop table"], schema.rollback());
/// ```
pub mod trait_object {
    pub trait Migration {
        fn execute(&self) -> &str;
        fn rollback(&self) -> &str;
    }

    pub struct CreateTable;
    impl Migration for CreateTable {
        fn execute(&self) -> &str {
            "create table"
        }

        fn rollback(&self) -> &str {
            "drop table"
        }
    }

    pub struct AddField;
    impl Migration for AddField {
        fn execute(&self) -> &str {
            "add field"
        }

        fn rollback(&self) -> &str {
            "remove field"
        }
    }

    pub struct Schema {
        commands: Vec<Box<dyn Migration>>,
    }
    impl Schema {
        pub fn new() -> Self {
            Schema {
                commands: Vec::new(),
            }
        }

        pub fn add_migration(&mut self, cmd: Box<dyn Migration>) {
            self.commands.push(cmd);
        }

        pub fn execute(&self) -> Vec<&str> {
            self.commands.iter().map(|cmd| cmd.execute()).collect()
        }

        pub fn rollback(&self) -> Vec<&str> {
            self.commands
                .iter()
                .rev()
                .map(|cmd| cmd.rollback())
                .collect()
        }
    }

    impl Default for Schema {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// If commands are small and may be defined as functions or passed as a closure
/// then using this might be preferable since it does not exploit dynamic dispatch
///
/// # Example
/// ```
/// # use design_patterns::patterns::command::function_pointer::FnPtr;
/// # fn use_function_pointer(add_field: FnPtr, remove_field: FnPtr) {
/// use design_patterns::patterns::command::function_pointer::Schema;
///
/// let mut schema = Schema::default();
///
/// schema.add_migration(
///     || String::from("create table"),
///     || String::from("drop table"),
/// );
/// schema.add_migration(add_field, remove_field);
///
/// assert_eq!(vec!["create table", "add field"], schema.execute());
/// assert_eq!(vec!["remove field", "drop table"], schema.rollback());
/// # }
/// ```
pub mod function_pointer {
    pub type FnPtr = fn() -> String;

    pub struct Command {
        execute: FnPtr,
        rollback: FnPtr,
    }

    pub struct Schema {
        commands: Vec<Command>,
    }

    impl Schema {
        pub fn new() -> Self {
            Schema {
                commands: Vec::new(),
            }
        }

        pub fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
            self.commands.push(Command { execute, rollback });
        }

        pub fn execute(&self) -> Vec<String> {
            self.commands.iter().map(|cmd| (cmd.execute)()).collect()
        }

        pub fn rollback(&self) -> Vec<String> {
            self.commands
                .iter()
                .rev()
                .map(|cmd| (cmd.rollback)())
                .collect()
        }
    }

    impl Default for Schema {
        fn default() -> Self {
            Self::new()
        }
    }
}

mod fn_trait_object {
    type Migration<'a> = Box<dyn Fn() -> &'a str>;

    pub struct Schema<'a> {
        executes: Vec<Migration<'a>>,
        rollbacks: Vec<Migration<'a>>,
    }

    impl<'a> Schema<'a> {
        pub fn new() -> Self {
            Schema {
                executes: Vec::new(),
                rollbacks: Vec::new(),
            }
        }

        pub fn add_migration<E, R>(&mut self, execute: E, rollback: R)
        where
            E: Fn() -> &'a str + 'static,
            R: Fn() -> &'a str + 'static,
        {
            self.executes.push(Box::new(execute));
            self.rollbacks.push(Box::new(rollback));
        }

        pub fn execute(&self) -> Vec<&str> {
            self.executes.iter().map(|execute| execute()).collect()
        }

        pub fn rollback(&self) -> Vec<&str> {
            self.rollbacks
                .iter()
                .rev()
                .map(|rollback| rollback())
                .collect()
        }
    }

    impl<'a> Default for Schema<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod test_trait_object {
    use super::trait_object::{AddField, CreateTable, Schema};

    #[test]
    fn test_command() {
        let mut schema = Schema::default();
        let cmd = Box::new(CreateTable);

        schema.add_migration(cmd);

        let cmd = Box::new(AddField);

        schema.add_migration(cmd);

        assert_eq!(vec!["create table", "add field"], schema.execute());
        assert_eq!(vec!["remove field", "drop table"], schema.rollback());
    }
}

#[cfg(test)]
mod test_function_pointer {
    use super::function_pointer::Schema;

    fn add_field() -> String {
        String::from("add field")
    }

    fn remove_field() -> String {
        String::from("remove field")
    }

    #[test]
    fn test_command() {
        let mut schema = Schema::default();

        schema.add_migration(
            || String::from("create table"),
            || String::from("drop table"),
        );
        schema.add_migration(add_field, remove_field);

        assert_eq!(vec!["create table", "add field"], schema.execute());
        assert_eq!(vec!["remove field", "drop table"], schema.rollback());
    }
}

#[cfg(test)]
mod test_fn_trait_object {
    use crate::patterns::command::fn_trait_object::Schema;

    fn add_field() -> &'static str {
        "add field"
    }

    fn remove_field() -> &'static str {
        "remove field"
    }

    #[test]
    fn test_command() {
        let mut schema = Schema::default();

        schema.add_migration(|| "create table", || "drop table");
        schema.add_migration(add_field, remove_field);

        assert_eq!(vec!["create table", "add field"], schema.execute());
        assert_eq!(vec!["remove field", "drop table"], schema.rollback());
    }
}
