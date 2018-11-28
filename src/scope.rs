/// Use this enum to represent to two kinds of 'scopes' of effects: Local and Global.
/// This is most useful in the case of config files.
///
/// For any given machine, there are two active config files: one local
/// to the machine, named as [hostname].dot.json, and those which are common
/// to all machines, known as simply .dot.json
pub enum Scope {
    Local,
    Global
}

