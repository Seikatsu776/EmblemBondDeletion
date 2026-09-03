use engage::app::eventscript::EventScript;
use engage::app::godpool::GodPool;
use engage::app::godunit::IGodUnit;
use engage::app::godbondholder::IGodBondHolderMethods;
use engage::moon_sharp::interpreter::dynvalue::DynValue;
use engage::prelude::*;

#[skyline::main(name = "delete_guest_bond")]
pub fn main()
{
     std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        let err_msg = format!(
            "Example plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            190,
            "delete_guest_bond plugin has panicked! Sorry it failed. Please let me know where and when this crashed.\n\0",
            err_msg.as_str(),
        );
    }));

    cobapi::install_lua_command_registerer( register_emblem_commands );
}

extern "C" fn register_emblem_commands( script: EventScript )
{
    script.register_action( "GodBondDelete", god_bond_delete );
}

extern "C" fn god_bond_delete( args: Array<DynValue>, _method_info: OptionalMethod )
{
    let gid = args.try_get_string( 0 );

    if gid.is_null()
    {
        return;
    }

    let god_unit = GodPool::try_get( gid, false );

    if god_unit.is_null()
    {
        return;
    }

    let bond_holder = god_unit.m_bonds();

    if bond_holder.is_null()
    {
        return;
    }

    bond_holder.clear();
}
