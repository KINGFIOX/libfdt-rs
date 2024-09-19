//! Definitions of structs and enums from the device tree specification.
//! reference to https://github.com/riscv-software-src/opensbi/blob/master/lib/utils/libfdt/libfdt.h

pub enum FdtErr {
    InfoErr(InfoErr),
    ParamErr(ParamErr),
    DTBErr(DTBErr),
    /*  FDT_ERR_INTERNAL: libfdt has failed an internal assertion.
     *  Should never be returned, if it is, it indicates a bug in
     *  libfdt itself. */
    Internal,
    ContentErr(ContentErr),
}

/// informative error codes
pub enum InfoErr {
    /// The requested node or property does not exist
    NotFound,
    /// Attempted to create a node or property which
    /// already exists
    Exists,
    /// Operation needed to expand the device
    /// tree, but its buffer did not have sufficient space to
    /// contain the expanded tree. Use fdt_open_into() to move the
    /// device tree to a buffer with more space.
    NoSpace,
}

/// codes for bad parameters
pub enum ParamErr {
    /// Function was passed a structure block
    /// offset which is out-of-bounds, or which points to an
    /// unsuitable part of the structure for the operation.
    BadOffset,
    /// Function was passed a badly formatted path
    /// (e.g. missing a leading / for a function which requires an
    /// absolute path)
    BadPath,
    /// Function was passed an invalid phandle.
    /// This can be caused either by an invalid phandle property
    /// length, or the phandle value was either 0 or -1, which are
    /// not permitted.
    BadPHandle,
    /// Function was passed an incomplete device
    /// tree created by the sequential-write functions, which is
    /// not sufficiently complete for the requested operation.
    BadState,
}

/// codes for bad device tree blobs
pub enum DTBErr {
    /// FDT or a sub-block is improperly
    /// terminated (overflows, goes outside allowed bounds, or
    /// isn't properly terminated).
    Truncated,
    ///  Given "device tree" appears not to be a
    /// device tree at all - it is missing the flattened device
    /// tree magic number.
    BadMagic,
    ///  Given device tree has a version which
    /// can't be handled by the requested operation.  For
    /// read-write functions, this may mean that fdt_open_into() is
    /// required to convert the tree to the expected version.
    BadVersion,
    /// Given device tree has a corrupt
    /// structure block or other serious error (e.g. misnested
    /// nodes, or subnodes preceding properties).
    BadStructure,
    /// For read-write functions, the given
    /// device tree has it's sub-blocks in an order that the
    /// function can't handle (memory reserve map, then structure,
    /// then strings).  Use fdt_open_into() to reorganize the tree
    /// into a form suitable for the read-write operations.
    BadLayout,
}

/// Errors in device tree content
pub enum ContentErr {
    /// Device tree has a #address-cells, #size-cells
    /// or similar property with a bad format or value
    BadNCells,
    /// Device tree has a property with an unexpected
    /// value. For example: a property expected to contain a string list
    /// is not NUL-terminated within the length of its value.
    BadValue,
    /// The device tree overlay, while
    /// correctly structured, cannot be applied due to some
    /// unexpected or missing value, property or node.
    BadOverlay,
    /// The device tree doesn't have any
    /// phandle available anymore without causing an overflow
    NoPHandles,
    /// The function was passed a flags field that
    /// contains invalid flags or an invalid combination of flags.
    BadFlags,
    /// The device tree base address is not 8-byte aligned.
    Alignment,
}

impl From<InfoErr> for FdtErr {
    fn from(value: InfoErr) -> Self {
        Self::InfoErr(value)
    }
}

impl From<ParamErr> for FdtErr {
    fn from(value: ParamErr) -> Self {
        Self::ParamErr(value)
    }
}

impl From<DTBErr> for FdtErr {
    fn from(value: DTBErr) -> Self {
        Self::DTBErr(value)
    }
}

impl From<ContentErr> for FdtErr {
    fn from(value: ContentErr) -> Self {
        Self::ContentErr(value)
    }
}
