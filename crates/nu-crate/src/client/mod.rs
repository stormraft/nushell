mod buffle;

pub use buffle::Buffle;

use nu_protocol::engine::StateWorkingSet;

#[allow(dead_code)]
pub fn add_client_decls(working_set: &mut StateWorkingSet) {
    macro_rules! bind_command {
        ( $command:expr ) => {
            working_set.add_decl(Box::new($command));
        };
        ( $( $command:expr ),* ) => {
            $( working_set.add_decl(Box::new($command)); )*
        };
    }

    // Client commands
    bind_command!(Buffle);
}
