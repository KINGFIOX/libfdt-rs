pub mod assume {
    #[allow(clippy::too_long_first_doc_paragraph)]
    /// This does essentially no checks. Only the latest device-tree
    /// version is correctly handled. Inconsistencies or errors in the device
    /// tree may cause undefined behavior or crashes. Invalid parameters
    /// passed to libfdt may do the same.
    ///
    /// If an error occurs when modifying the tree it may leave the tree in
    /// an intermediate (but valid) state. As an example, adding a property
    /// where there is insufficient space may result in the property name
    /// being added to the string table even though the property itself is
    /// not added to the struct section.
    ///
    /// Only use this if you have a fully validated device tree with
    /// the latest supported version and wish to minimize code size.
    pub const PERFECT: u32 = 0xff;

    /// This assumes that the device tree is sane. i.e. header metadata
    /// and basic hierarchy are correct.
    ///
    /// With this assumption enabled, normal device trees produced by libfdt
    /// and the compiler should be handled safely. Malicious device trees and
    /// complete garbage may cause libfdt to behave badly or crash. Truncated
    /// device trees (e.g. those only partially loaded) can also cause
    /// problems.
    ///
    /// Note: Only checks that relate exclusively to the device tree itself
    /// (not the parameters passed to libfdt) are disabled by this
    /// assumption. This includes checking headers, tags and the like.
    pub const VALID_DTB: u32 = 1 << 0;

    #[allow(clippy::too_long_first_doc_paragraph)]
    /// This builds on ASSUME_VALID_DTB and further assumes that libfdt
    /// functions are called with valid parameters, i.e. not trigger
    /// FDT_ERR_BADOFFSET or offsets that are out of bounds. It disables any
    /// extensive checking of parameters and the device tree, making various
    /// assumptions about correctness.
    ///
    /// It doesn't make sense to enable this assumption unless
    /// ASSUME_VALID_DTB is also enabled.
    pub const VALID_INPUT: u32 = 1 << 1;

    /// This disables checks for device-tree version and removes all code
    /// which handles older versions.
    ///
    /// Only enable this if you know you have a device tree with the latest
    /// version.
    pub const LATEST: u32 = 1 << 2;

    #[allow(clippy::too_long_first_doc_paragraph)]
    /// This assumes that it is OK for a failed addition to the device tree,
    /// due to lack of space or some other problem, to skip any rollback
    /// steps (such as dropping the property name from the string table).
    /// This is safe to enable in most circumstances, even though it may
    /// leave the tree in a sub-optimal state.
    pub const ASSUME_NO_ROLLBACK: u32 = 1 << 3;

    /// This assumes that the device tree components appear in a 'convenient'
    /// order, i.e. the memory reservation block first, then the structure
    /// block and finally the string block.
    ///
    /// This order is not specified by the device-tree specification,
    /// but is expected by libfdt. The device-tree compiler always created
    /// device trees with this order.
    ///
    /// This assumption disables a check in fdt_open_into() and removes the
    /// ability to fix the problem there. This is safe if you know that the
    /// device tree is correctly ordered. See fdt_blocks_misordered_().
    pub const ASSUME_LIBFDT_ORDER: u32 = 1 << 4;

    /*
     * This assumes that libfdt itself does not have any internal bugs. It
     * drops certain checks that should never be needed unless libfdt has an
     * undiscovered bug.
     *
     * This can generally be considered safe to enable.
     */
    pub const ASSUME_LIBFDT_FLAWLESS: u32 = 1 << 5;
}
