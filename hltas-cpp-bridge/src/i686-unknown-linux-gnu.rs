/* automatically generated by rust-bindgen 0.59.1 */

pub type size_t = ::std::os::raw::c_uint;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
extern "C" {
    pub fn hltas_input_set_property(
        input: *mut ::std::os::raw::c_void,
        property: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn hltas_input_get_property(
        input: *const ::std::os::raw::c_void,
        property: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn hltas_input_push_frame(input: *mut ::std::os::raw::c_void, frame: *const hltas_frame);
}
extern "C" {
    pub fn hltas_input_get_frame(
        input: *const ::std::os::raw::c_void,
        index: size_t,
        frame: *mut hltas_frame,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hltas_input_set_error_message(
        input: *mut ::std::os::raw::c_void,
        message: *const ::std::os::raw::c_char,
    );
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ErrorCode {
    OK = 0,
    FAILOPEN = 1,
    FAILVER = 2,
    NOTSUPPORTED = 3,
    FAILLINE = 4,
    NOSAVENAME = 5,
    FAILFRAME = 6,
    FAILWRITE = 7,
    NOSEED = 8,
    NOYAW = 9,
    NOBUTTONS = 10,
    BOTHAJDT = 11,
    NOLGAGSTACTION = 12,
    NOLGAGSTMINSPEED = 13,
    LGAGSTACTIONTIMES = 14,
    NORESETSEED = 15,
    INVALID_ALGORITHM = 16,
    MISSING_CONSTRAINTS = 17,
    NO_PM_IN_TOLERANCE = 18,
    MISSING_ALGORITHM_FROMTO_PARAMETERS = 19,
    NO_TO_IN_FROMTO_ALGORITHM = 20,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ErrorDescription {
    pub Code: ErrorCode,
    pub LineNumber: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_ErrorDescription() {
    assert_eq!(
        ::std::mem::size_of::<ErrorDescription>(),
        8usize,
        concat!("Size of: ", stringify!(ErrorDescription))
    );
    assert_eq!(
        ::std::mem::align_of::<ErrorDescription>(),
        4usize,
        concat!("Alignment of ", stringify!(ErrorDescription))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorDescription>())).Code as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorDescription),
            "::",
            stringify!(Code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorDescription>())).LineNumber as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorDescription),
            "::",
            stringify!(LineNumber)
        )
    );
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StrafeType {
    MAXACCEL = 0,
    MAXANGLE = 1,
    MAXDECCEL = 2,
    CONSTSPEED = 3,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StrafeDir {
    LEFT = 0,
    RIGHT = 1,
    BEST = 2,
    YAW = 3,
    POINT = 4,
    LINE = 5,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ButtonState {
    NOTHING = 0,
    SET = 1,
    CLEAR = 2,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Button {
    FORWARD = 0,
    FORWARD_LEFT = 1,
    LEFT = 2,
    BACK_LEFT = 3,
    BACK = 4,
    BACK_RIGHT = 5,
    RIGHT = 6,
    FORWARD_RIGHT = 7,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StrafeButtons {
    pub AirLeft: Button,
    pub AirRight: Button,
    pub GroundLeft: Button,
    pub GroundRight: Button,
}
#[test]
fn bindgen_test_layout_StrafeButtons() {
    assert_eq!(
        ::std::mem::size_of::<StrafeButtons>(),
        4usize,
        concat!("Size of: ", stringify!(StrafeButtons))
    );
    assert_eq!(
        ::std::mem::align_of::<StrafeButtons>(),
        1usize,
        concat!("Alignment of ", stringify!(StrafeButtons))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<StrafeButtons>())).AirLeft as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StrafeButtons),
            "::",
            stringify!(AirLeft)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<StrafeButtons>())).AirRight as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(StrafeButtons),
            "::",
            stringify!(AirRight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<StrafeButtons>())).GroundLeft as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(StrafeButtons),
            "::",
            stringify!(GroundLeft)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<StrafeButtons>())).GroundRight as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(StrafeButtons),
            "::",
            stringify!(GroundRight)
        )
    );
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StrafingAlgorithm {
    YAW = 0,
    VECTORIAL = 1,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ConstraintsType {
    VELOCITY = 0,
    VELOCITY_AVG = 1,
    VELOCITY_LOCK = 2,
    YAW = 3,
    YAW_RANGE = 4,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlgorithmParameters {
    pub Type: ConstraintsType,
    pub Parameters: AlgorithmParameters__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AlgorithmParameters__bindgen_ty_1 {
    pub Velocity: AlgorithmParameters__bindgen_ty_1__bindgen_ty_1,
    pub VelocityAvg: AlgorithmParameters__bindgen_ty_1__bindgen_ty_2,
    pub VelocityLock: AlgorithmParameters__bindgen_ty_1__bindgen_ty_3,
    pub Yaw: AlgorithmParameters__bindgen_ty_1__bindgen_ty_4,
    pub YawRange: AlgorithmParameters__bindgen_ty_1__bindgen_ty_5,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_1 {
    pub Constraints: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_1>())).Constraints
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Constraints)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_2 {
    pub Constraints: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_2>())).Constraints
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(Constraints)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_3 {
    pub Constraints: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_3>())).Constraints
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(Constraints)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_4 {
    pub Yaw: f64,
    pub Constraints: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_4>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_4>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_4>())).Yaw
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(Yaw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_4>())).Constraints
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(Constraints)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlgorithmParameters__bindgen_ty_1__bindgen_ty_5 {
    pub LowestYaw: f64,
    pub HighestYaw: f64,
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_5>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_5>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_5>())).LowestYaw
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(LowestYaw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1__bindgen_ty_5>())).HighestYaw
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(HighestYaw)
        )
    );
}
#[test]
fn bindgen_test_layout_AlgorithmParameters__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(AlgorithmParameters__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(AlgorithmParameters__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1>())).Velocity as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(Velocity)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1>())).VelocityAvg as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(VelocityAvg)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1>())).VelocityLock as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(VelocityLock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1>())).Yaw as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(Yaw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlgorithmParameters__bindgen_ty_1>())).YawRange as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters__bindgen_ty_1),
            "::",
            stringify!(YawRange)
        )
    );
}
#[test]
fn bindgen_test_layout_AlgorithmParameters() {
    assert_eq!(
        ::std::mem::size_of::<AlgorithmParameters>(),
        20usize,
        concat!("Size of: ", stringify!(AlgorithmParameters))
    );
    assert_eq!(
        ::std::mem::align_of::<AlgorithmParameters>(),
        4usize,
        concat!("Alignment of ", stringify!(AlgorithmParameters))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlgorithmParameters>())).Type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters),
            "::",
            stringify!(Type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlgorithmParameters>())).Parameters as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlgorithmParameters),
            "::",
            stringify!(Parameters)
        )
    );
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ChangeTarget {
    YAW = 0,
    PITCH = 1,
    TARGET_YAW = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hltas_frame {
    pub Strafe: bool,
    pub Lgagst: bool,
    pub Autojump: bool,
    pub Ducktap: bool,
    pub Jumpbug: bool,
    pub Dbc: bool,
    pub Dbg: bool,
    pub Dwj: bool,
    pub Type: StrafeType,
    pub Dir: StrafeDir,
    pub LgagstFullMaxspeed: bool,
    pub LgagstTimes: u32,
    pub AutojumpTimes: u32,
    pub Ducktap0ms: bool,
    pub DucktapTimes: u32,
    pub JumpbugTimes: u32,
    pub DbcCeilings: bool,
    pub DbcTimes: u32,
    pub DbgTimes: u32,
    pub DwjTimes: u32,
    pub Forward: bool,
    pub Left: bool,
    pub Right: bool,
    pub Back: bool,
    pub Up: bool,
    pub Down: bool,
    pub Jump: bool,
    pub Duck: bool,
    pub Use: bool,
    pub Attack1: bool,
    pub Attack2: bool,
    pub Reload: bool,
    pub Frametime: *const ::std::os::raw::c_char,
    pub PitchPresent: bool,
    pub YawPresent: bool,
    pub Yaw: f64,
    pub X: f64,
    pub Y: f64,
    pub Pitch: f64,
    pub Repeats: u32,
    pub Commands: *const ::std::os::raw::c_char,
    pub Comments: *const ::std::os::raw::c_char,
    pub SaveName: *const ::std::os::raw::c_char,
    pub SeedPresent: bool,
    pub Seed: u32,
    pub BtnState: ButtonState,
    pub Buttons: StrafeButtons,
    pub LgagstMinSpeedPresent: bool,
    pub LgagstMinSpeed: f32,
    pub ResetFrame: bool,
    pub ResetNonSharedRNGSeed: i64,
    pub StrafingAlgorithmPresent: bool,
    pub Algorithm: StrafingAlgorithm,
    pub AlgorithmParametersPresent: bool,
    pub Parameters: AlgorithmParameters,
    pub ChangePresent: bool,
    pub Target: ChangeTarget,
    pub ChangeFinalValue: f32,
    pub ChangeOver: f32,
    pub TargetYawOverride: *const f32,
    pub TargetYawOverrideCount: size_t,
}
#[test]
fn bindgen_test_layout_hltas_frame() {
    assert_eq!(
        ::std::mem::size_of::<hltas_frame>(),
        200usize,
        concat!("Size of: ", stringify!(hltas_frame))
    );
    assert_eq!(
        ::std::mem::align_of::<hltas_frame>(),
        4usize,
        concat!("Alignment of ", stringify!(hltas_frame))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Strafe as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Strafe)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Lgagst as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Lgagst)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Autojump as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Autojump)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Ducktap as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Ducktap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Jumpbug as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Jumpbug)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Dbc as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Dbc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Dbg as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Dbg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Dwj as *const _ as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Dwj)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Type as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Dir as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Dir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).LgagstFullMaxspeed as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(LgagstFullMaxspeed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).LgagstTimes as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(LgagstTimes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).AutojumpTimes as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(AutojumpTimes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Ducktap0ms as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Ducktap0ms)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).DucktapTimes as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(DucktapTimes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).JumpbugTimes as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(JumpbugTimes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).DbcCeilings as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(DbcCeilings)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).DbcTimes as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(DbcTimes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).DbgTimes as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(DbgTimes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).DwjTimes as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(DwjTimes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Forward as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Forward)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Left as *const _ as usize },
        49usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Left)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Right as *const _ as usize },
        50usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Right)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Back as *const _ as usize },
        51usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Back)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Up as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Up)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Down as *const _ as usize },
        53usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Down)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Jump as *const _ as usize },
        54usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Jump)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Duck as *const _ as usize },
        55usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Duck)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Use as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Use)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Attack1 as *const _ as usize },
        57usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Attack1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Attack2 as *const _ as usize },
        58usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Attack2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Reload as *const _ as usize },
        59usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Reload)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Frametime as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Frametime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).PitchPresent as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(PitchPresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).YawPresent as *const _ as usize },
        65usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(YawPresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Yaw as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Yaw)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).X as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(X)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Y as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Pitch as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Pitch)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Repeats as *const _ as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Repeats)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Commands as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Commands)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Comments as *const _ as usize },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Comments)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).SaveName as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(SaveName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).SeedPresent as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(SeedPresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Seed as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Seed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).BtnState as *const _ as usize },
        124usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(BtnState)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Buttons as *const _ as usize },
        125usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Buttons)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<hltas_frame>())).LgagstMinSpeedPresent as *const _ as usize
        },
        129usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(LgagstMinSpeedPresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).LgagstMinSpeed as *const _ as usize },
        132usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(LgagstMinSpeed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).ResetFrame as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(ResetFrame)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<hltas_frame>())).ResetNonSharedRNGSeed as *const _ as usize
        },
        140usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(ResetNonSharedRNGSeed)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<hltas_frame>())).StrafingAlgorithmPresent as *const _ as usize
        },
        148usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(StrafingAlgorithmPresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Algorithm as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Algorithm)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<hltas_frame>())).AlgorithmParametersPresent as *const _ as usize
        },
        156usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(AlgorithmParametersPresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Parameters as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Parameters)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).ChangePresent as *const _ as usize },
        180usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(ChangePresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).Target as *const _ as usize },
        181usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(Target)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).ChangeFinalValue as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(ChangeFinalValue)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).ChangeOver as *const _ as usize },
        188usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(ChangeOver)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hltas_frame>())).TargetYawOverride as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(TargetYawOverride)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<hltas_frame>())).TargetYawOverrideCount as *const _ as usize
        },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(hltas_frame),
            "::",
            stringify!(TargetYawOverrideCount)
        )
    );
}
