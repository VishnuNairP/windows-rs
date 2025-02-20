#[cfg(feature = "Globalization_Collation")]
pub mod Collation;
#[cfg(feature = "Globalization_DateTimeFormatting")]
pub mod DateTimeFormatting;
#[cfg(feature = "Globalization_Fonts")]
pub mod Fonts;
#[cfg(feature = "Globalization_NumberFormatting")]
pub mod NumberFormatting;
#[cfg(feature = "Globalization_PhoneNumberFormatting")]
pub mod PhoneNumberFormatting;
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationLanguagesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IApplicationLanguagesStatics {
    type Vtable = IApplicationLanguagesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IApplicationLanguagesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75b40847_0a4c_4a92_9565_fd63c95f7aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationLanguagesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrimaryLanguageOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPrimaryLanguageOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ManifestLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ManifestLanguages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationLanguagesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IApplicationLanguagesStatics2 {
    type Vtable = IApplicationLanguagesStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IApplicationLanguagesStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1df0de4f_072b_4d7b_8f06_cb2db40f2bb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationLanguagesStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub GetLanguagesForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    GetLanguagesForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendar(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICalendar {
    type Vtable = ICalendar_Vtbl;
}
unsafe impl ::windows::core::Interface for ICalendar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca30221d_86d9_40fb_a26b_d44eb7cf08ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendar_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetToMin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetToMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetCalendarSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ChangeCalendarSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ChangeClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDateTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDateTime: usize,
    pub SetToNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FirstEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LastEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfEras: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Era: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub AddEras: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eras: i32) -> ::windows::core::HRESULT,
    pub EraAsFullString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EraAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FirstYearInThisEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LastYearInThisEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfYearsInThisEra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub AddYears: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, years: i32) -> ::windows::core::HRESULT,
    pub YearAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub YearAsTruncatedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remainingdigits: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub YearAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FirstMonthInThisYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LastMonthInThisYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfMonthsInThisYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub AddMonths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, months: i32) -> ::windows::core::HRESULT,
    pub MonthAsFullString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MonthAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MonthAsFullSoloString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MonthAsSoloString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MonthAsNumericString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MonthAsPaddedNumericString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddWeeks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weeks: i32) -> ::windows::core::HRESULT,
    pub FirstDayInThisMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LastDayInThisMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfDaysInThisMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub AddDays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT,
    pub DayAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DayAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DayOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DayOfWeek) -> ::windows::core::HRESULT,
    pub DayOfWeekAsFullString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DayOfWeekAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DayOfWeekAsFullSoloString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DayOfWeekAsSoloString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FirstPeriodInThisDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LastPeriodInThisDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfPeriodsInThisDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Period: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub AddPeriods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, periods: i32) -> ::windows::core::HRESULT,
    pub PeriodAsFullString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PeriodAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FirstHourInThisPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LastHourInThisPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfHoursInThisPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Hour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub AddHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hours: i32) -> ::windows::core::HRESULT,
    pub HourAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HourAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Minute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetMinute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub AddMinutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub MinuteAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MinuteAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Second: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub AddSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: i32) -> ::windows::core::HRESULT,
    pub SecondAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecondAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Nanosecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetNanosecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub AddNanoseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nanoseconds: i32) -> ::windows::core::HRESULT,
    pub NanosecondAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NanosecondAsPaddedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindigits: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, other: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompareDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, other: super::Foundation::DateTime, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompareDateTime: usize,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, other: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FirstMinuteInThisHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LastMinuteInThisHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfMinutesInThisHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub FirstSecondInThisMinute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LastSecondInThisMinute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfSecondsInThisMinute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsDaylightSavingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICalendarFactory {
    type Vtable = ICalendarFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICalendarFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83f58412_e56b_4c75_a66e_0f63d57758a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCalendarDefaultCalendarAndClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCalendarDefaultCalendarAndClock: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCalendar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void, calendar: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCalendar: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICalendarFactory2 {
    type Vtable = ICalendarFactory2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICalendarFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb44b378c_ca7e_4590_9e72_ea2bec1a5115);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarFactory2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCalendarWithTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void, calendar: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, timezoneid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCalendarWithTimeZone: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICalendarIdentifiersStatics {
    type Vtable = ICalendarIdentifiersStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICalendarIdentifiersStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80653f68_2cb2_4c1f_b590_f0f52bf4fd1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarIdentifiersStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Gregorian: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Hebrew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Hijri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Japanese: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Julian: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Korean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Taiwan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Thai: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UmAlQura: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICalendarIdentifiersStatics2 {
    type Vtable = ICalendarIdentifiersStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICalendarIdentifiersStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7df4d488_5fd0_42a7_95b5_7d98d823075f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarIdentifiersStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Persian: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarIdentifiersStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICalendarIdentifiersStatics3 {
    type Vtable = ICalendarIdentifiersStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for ICalendarIdentifiersStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c225423_1fad_40c0_9334_a8eb90db04f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarIdentifiersStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChineseLunar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub JapaneseLunar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KoreanLunar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TaiwanLunar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VietnameseLunar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClockIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClockIdentifiersStatics {
    type Vtable = IClockIdentifiersStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IClockIdentifiersStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x523805bb_12ec_4f83_bc31_b1b4376b0808);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClockIdentifiersStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TwelveHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TwentyFourHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyAmount(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICurrencyAmount {
    type Vtable = ICurrencyAmount_Vtbl;
}
unsafe impl ::windows::core::Interface for ICurrencyAmount {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74b49942_eb75_443a_95b3_7d723f56f93c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyAmount_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Amount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Currency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyAmountFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICurrencyAmountFactory {
    type Vtable = ICurrencyAmountFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICurrencyAmountFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48d7168f_ef3b_4aee_a6a1_4b036fe03ff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyAmountFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, currency: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICurrencyIdentifiersStatics {
    type Vtable = ICurrencyIdentifiersStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICurrencyIdentifiersStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f1d091b_d586_4913_9b6a_a9bd2dc12874);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyIdentifiersStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AED: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AFN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ALL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AMD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ANG: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AOA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ARS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AUD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AWG: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AZN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BAM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BBD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BDT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BGN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BHD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BIF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BMD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BOB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BRL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BSD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BTN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BWP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BYR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BZD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CAD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CDF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CHF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CLP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CNY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub COP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CRC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CUP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CVE: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CZK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DJF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DKK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DOP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DZD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EGP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ERN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ETB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EUR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FJD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FKP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GBP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GEL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GHS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GIP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GMD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GNF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GTQ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GYD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HKD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HNL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HRK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HTG: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HUF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IDR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ILS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub INR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IQD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IRR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ISK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub JMD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub JOD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub JPY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KES: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KGS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KHR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KMF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KPW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KRW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KWD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KYD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KZT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LAK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LBP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LKR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LRD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LSL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LVL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LYD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MAD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MDL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MGA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MKD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MMK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MNT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MOP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MRO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MUR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MVR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MWK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MXN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MYR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MZN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NAD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NGN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NOK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NPR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NZD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub OMR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PAB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PEN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PGK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PHP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PKR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PLN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PYG: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub QAR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RON: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RSD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RUB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RWF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SAR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SBD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SCR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SDG: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SEK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SGD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SHP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SLL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SOS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SRD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub STD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SYP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SZL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub THB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TJS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TMT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TOP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TRY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TTD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TWD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TZS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UAH: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UGX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub USD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UYU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UZS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VEF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VUV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WST: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XAF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XCD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XPF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XXX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub YER: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ZAR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ZMW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ZWL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICurrencyIdentifiersStatics2 {
    type Vtable = ICurrencyIdentifiersStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICurrencyIdentifiersStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1814797f_c3b2_4c33_9591_980011950d37);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyIdentifiersStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BYN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyIdentifiersStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICurrencyIdentifiersStatics3 {
    type Vtable = ICurrencyIdentifiersStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for ICurrencyIdentifiersStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fb23bfa_ed25_4f4d_857f_237f1748c21c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyIdentifiersStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MRU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SSP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub STN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VES: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeographicRegion(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeographicRegion {
    type Vtable = IGeographicRegion_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeographicRegion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01e9a621_4a64_4ed9_954f_9edeb07bd903);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeographicRegion_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CodeTwoLetter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CodeThreeLetter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CodeThreeDigit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NativeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CurrenciesInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CurrenciesInUse: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeographicRegionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeographicRegionFactory {
    type Vtable = IGeographicRegionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeographicRegionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53425270_77b4_426b_859f_81e19d512546);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeographicRegionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geographicregioncode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeographicRegionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeographicRegionStatics {
    type Vtable = IGeographicRegionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeographicRegionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29e28974_7ad9_4ef4_8799_b3b44fadec08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeographicRegionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geographicregioncode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJapanesePhoneme(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJapanesePhoneme {
    type Vtable = IJapanesePhoneme_Vtbl;
}
unsafe impl ::windows::core::Interface for IJapanesePhoneme {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f6a9300_e85b_43e6_897d_5d82f862df21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJapanesePhoneme_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub YomiText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsPhraseStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJapanesePhoneticAnalyzerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJapanesePhoneticAnalyzerStatics {
    type Vtable = IJapanesePhoneticAnalyzerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IJapanesePhoneticAnalyzerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88ab9e90_93de_41b2_b4d5_8edb227fd1c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJapanesePhoneticAnalyzerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetWords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetWords: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetWordsWithMonoRubyOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, monoruby: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetWordsWithMonoRubyOption: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILanguage {
    type Vtable = ILanguage_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea79a752_f7c2_4265_b1bd_c4dec4e4f080);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LanguageTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NativeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Script: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguage2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILanguage2 {
    type Vtable = ILanguage2_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a47e5b5_d94d_4886_a404_a5a5b9d5b494);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguage2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LayoutDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LanguageLayoutDirection) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguage3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILanguage3 {
    type Vtable = ILanguage3_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguage3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6af3d10_641a_5ba4_bb43_5e12aed75954);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguage3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AbbreviatedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageExtensionSubtags(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILanguageExtensionSubtags {
    type Vtable = ILanguageExtensionSubtags_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguageExtensionSubtags {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d7daf45_368d_4364_852b_dec927037b85);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExtensionSubtags_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetExtensionSubtags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, singleton: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetExtensionSubtags: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILanguageFactory {
    type Vtable = ILanguageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b0252ac_0c27_44f8_b792_9793fb66c63e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILanguageStatics {
    type Vtable = ILanguageStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguageStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb23cd557_0865_46d4_89b8_d59be8990f0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsWellFormed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CurrentInputMethodLanguageTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILanguageStatics2 {
    type Vtable = ILanguageStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguageStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30199f6e_914b_4b2a_9d6e_e3b0e27dbe4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TrySetInputMethodLanguageTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILanguageStatics3 {
    type Vtable = ILanguageStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for ILanguageStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd15ecb5a_71de_5752_9542_fac5b4f27261);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMuiCompatibleLanguageListFromLanguageTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetags: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMuiCompatibleLanguageListFromLanguageTags: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INumeralSystemIdentifiersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INumeralSystemIdentifiersStatics {
    type Vtable = INumeralSystemIdentifiersStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for INumeralSystemIdentifiersStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5c662c3_68c9_4d3d_b765_972029e21dec);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemIdentifiersStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Arab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ArabExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Bali: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Beng: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Cham: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Deva: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FullWide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Gujr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Guru: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HaniDec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Java: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Kali: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Khmr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Knda: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Lana: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LanaTham: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Laoo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Latn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Lepc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Limb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Mlym: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Mong: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Mtei: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Mymr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MymrShan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Nkoo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Olck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Orya: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Saur: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sund: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Talu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TamlDec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Telu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Thai: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Tibt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Vaii: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INumeralSystemIdentifiersStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INumeralSystemIdentifiersStatics2 {
    type Vtable = INumeralSystemIdentifiersStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for INumeralSystemIdentifiersStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f003228_9ddb_4a34_9104_0260c091a7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemIdentifiersStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Brah: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Osma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MathBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MathDbl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MathSans: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MathSanb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MathMono: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ZmthBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ZmthDbl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ZmthSans: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ZmthSanb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ZmthMono: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeZoneOnCalendar(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITimeZoneOnCalendar {
    type Vtable = ITimeZoneOnCalendar_Vtbl;
}
unsafe impl ::windows::core::Interface for ITimeZoneOnCalendar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb3c25e5_46cf_4317_a3f5_02621ad54478);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneOnCalendar_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ChangeTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timezoneid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TimeZoneAsFullString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TimeZoneAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ideallength: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct ApplicationLanguages;
impl ApplicationLanguages {
    pub fn PrimaryLanguageOverride() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IApplicationLanguagesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrimaryLanguageOverride)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SetPrimaryLanguageOverride(value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IApplicationLanguagesStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetPrimaryLanguageOverride)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages() -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IApplicationLanguagesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Languages)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ManifestLanguages() -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IApplicationLanguagesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManifestLanguages)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn GetLanguagesForUser(user: &super::System::User) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IApplicationLanguagesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetLanguagesForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApplicationLanguagesStatics<R, F: FnOnce(&IApplicationLanguagesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationLanguages, IApplicationLanguagesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationLanguagesStatics2<R, F: FnOnce(&IApplicationLanguagesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationLanguages, IApplicationLanguagesStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ApplicationLanguages {
    const NAME: &'static str = "Windows.Globalization.ApplicationLanguages";
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
pub struct Calendar(::windows::core::IUnknown);
impl Calendar {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Calendar, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Clone(&self) -> ::windows::core::Result<Calendar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<Calendar>(result__)
        }
    }
    pub fn SetToMin(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetToMin)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetToMax(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetToMax)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Languages)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumeralSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNumeralSystem)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetCalendarSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCalendarSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChangeCalendarSystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ChangeCalendarSystem)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetClock(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetClock)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChangeClock(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ChangeClock)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDateTime(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDateTime(&self, value: super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDateTime)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetToNow(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetToNow)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn FirstEra(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstEra)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn LastEra(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastEra)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn NumberOfEras(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumberOfEras)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Era(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Era)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetEra(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEra)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AddEras(&self, eras: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddEras)(::windows::core::Vtable::as_raw(this), eras).ok() }
    }
    pub fn EraAsFullString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EraAsFullString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn EraAsString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EraAsString)(::windows::core::Vtable::as_raw(this), ideallength, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FirstYearInThisEra(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstYearInThisEra)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn LastYearInThisEra(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastYearInThisEra)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn NumberOfYearsInThisEra(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumberOfYearsInThisEra)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Year(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Year)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetYear(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetYear)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AddYears(&self, years: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddYears)(::windows::core::Vtable::as_raw(this), years).ok() }
    }
    pub fn YearAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YearAsString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn YearAsTruncatedString(&self, remainingdigits: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YearAsTruncatedString)(::windows::core::Vtable::as_raw(this), remainingdigits, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn YearAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YearAsPaddedString)(::windows::core::Vtable::as_raw(this), mindigits, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FirstMonthInThisYear(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstMonthInThisYear)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn LastMonthInThisYear(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastMonthInThisYear)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn NumberOfMonthsInThisYear(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumberOfMonthsInThisYear)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Month(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Month)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetMonth(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMonth)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AddMonths(&self, months: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddMonths)(::windows::core::Vtable::as_raw(this), months).ok() }
    }
    pub fn MonthAsFullString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MonthAsFullString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MonthAsString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MonthAsString)(::windows::core::Vtable::as_raw(this), ideallength, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MonthAsFullSoloString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MonthAsFullSoloString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MonthAsSoloString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MonthAsSoloString)(::windows::core::Vtable::as_raw(this), ideallength, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MonthAsNumericString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MonthAsNumericString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MonthAsPaddedNumericString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MonthAsPaddedNumericString)(::windows::core::Vtable::as_raw(this), mindigits, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AddWeeks(&self, weeks: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddWeeks)(::windows::core::Vtable::as_raw(this), weeks).ok() }
    }
    pub fn FirstDayInThisMonth(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstDayInThisMonth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn LastDayInThisMonth(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastDayInThisMonth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn NumberOfDaysInThisMonth(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumberOfDaysInThisMonth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Day(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Day)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetDay(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDay)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AddDays(&self, days: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddDays)(::windows::core::Vtable::as_raw(this), days).ok() }
    }
    pub fn DayAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DayAsString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DayAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DayAsPaddedString)(::windows::core::Vtable::as_raw(this), mindigits, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DayOfWeek(&self) -> ::windows::core::Result<DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DayOfWeek)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<DayOfWeek>(result__)
        }
    }
    pub fn DayOfWeekAsFullString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DayOfWeekAsFullString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DayOfWeekAsString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DayOfWeekAsString)(::windows::core::Vtable::as_raw(this), ideallength, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DayOfWeekAsFullSoloString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DayOfWeekAsFullSoloString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DayOfWeekAsSoloString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DayOfWeekAsSoloString)(::windows::core::Vtable::as_raw(this), ideallength, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FirstPeriodInThisDay(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstPeriodInThisDay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn LastPeriodInThisDay(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastPeriodInThisDay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn NumberOfPeriodsInThisDay(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumberOfPeriodsInThisDay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Period(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Period)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetPeriod(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPeriod)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AddPeriods(&self, periods: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddPeriods)(::windows::core::Vtable::as_raw(this), periods).ok() }
    }
    pub fn PeriodAsFullString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PeriodAsFullString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn PeriodAsString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PeriodAsString)(::windows::core::Vtable::as_raw(this), ideallength, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FirstHourInThisPeriod(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstHourInThisPeriod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn LastHourInThisPeriod(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastHourInThisPeriod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn NumberOfHoursInThisPeriod(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumberOfHoursInThisPeriod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Hour(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Hour)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetHour(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHour)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AddHours(&self, hours: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddHours)(::windows::core::Vtable::as_raw(this), hours).ok() }
    }
    pub fn HourAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HourAsString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn HourAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HourAsPaddedString)(::windows::core::Vtable::as_raw(this), mindigits, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Minute(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Minute)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetMinute(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMinute)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AddMinutes(&self, minutes: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddMinutes)(::windows::core::Vtable::as_raw(this), minutes).ok() }
    }
    pub fn MinuteAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinuteAsString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MinuteAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinuteAsPaddedString)(::windows::core::Vtable::as_raw(this), mindigits, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Second(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Second)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetSecond(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSecond)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AddSeconds(&self, seconds: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddSeconds)(::windows::core::Vtable::as_raw(this), seconds).ok() }
    }
    pub fn SecondAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SecondAsString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SecondAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SecondAsPaddedString)(::windows::core::Vtable::as_raw(this), mindigits, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Nanosecond(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Nanosecond)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetNanosecond(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNanosecond)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AddNanoseconds(&self, nanoseconds: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddNanoseconds)(::windows::core::Vtable::as_raw(this), nanoseconds).ok() }
    }
    pub fn NanosecondAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NanosecondAsString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn NanosecondAsPaddedString(&self, mindigits: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NanosecondAsPaddedString)(::windows::core::Vtable::as_raw(this), mindigits, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Compare(&self, other: &Calendar) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compare)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(other), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompareDateTime(&self, other: super::Foundation::DateTime) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompareDateTime)(::windows::core::Vtable::as_raw(this), other, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn CopyTo(&self, other: &Calendar) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CopyTo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(other)).ok() }
    }
    pub fn FirstMinuteInThisHour(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstMinuteInThisHour)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn LastMinuteInThisHour(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastMinuteInThisHour)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn NumberOfMinutesInThisHour(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumberOfMinutesInThisHour)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn FirstSecondInThisMinute(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstSecondInThisMinute)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn LastSecondInThisMinute(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastSecondInThisMinute)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn NumberOfSecondsInThisMinute(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumberOfSecondsInThisMinute)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResolvedLanguage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsDaylightSavingTime(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDaylightSavingTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCalendarDefaultCalendarAndClock<'a, P0, E0>(languages: P0) -> ::windows::core::Result<Calendar>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICalendarFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCalendarDefaultCalendarAndClock)(::windows::core::Vtable::as_raw(this), languages.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<Calendar>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCalendar<'a, P0, E0>(languages: P0, calendar: &::windows::core::HSTRING, clock: &::windows::core::HSTRING) -> ::windows::core::Result<Calendar>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICalendarFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCalendar)(::windows::core::Vtable::as_raw(this), languages.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(calendar), ::core::mem::transmute_copy(clock), result__.as_mut_ptr()).from_abi::<Calendar>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCalendarWithTimeZone<'a, P0, E0>(languages: P0, calendar: &::windows::core::HSTRING, clock: &::windows::core::HSTRING, timezoneid: &::windows::core::HSTRING) -> ::windows::core::Result<Calendar>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICalendarFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCalendarWithTimeZone)(::windows::core::Vtable::as_raw(this), languages.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(calendar), ::core::mem::transmute_copy(clock), ::core::mem::transmute_copy(timezoneid), result__.as_mut_ptr()).from_abi::<Calendar>(result__)
        })
    }
    pub fn GetTimeZone(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTimeZone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChangeTimeZone(&self, timezoneid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ChangeTimeZone)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(timezoneid)).ok() }
    }
    pub fn TimeZoneAsFullString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeZoneAsFullString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TimeZoneAsString(&self, ideallength: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeZoneAsString)(::windows::core::Vtable::as_raw(this), ideallength, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICalendarFactory<R, F: FnOnce(&ICalendarFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Calendar, ICalendarFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICalendarFactory2<R, F: FnOnce(&ICalendarFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Calendar, ICalendarFactory2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Calendar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Calendar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Calendar {}
impl ::core::fmt::Debug for Calendar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Calendar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Calendar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.Calendar;{ca30221d-86d9-40fb-a26b-d44eb7cf08ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Calendar {
    type Vtable = ICalendar_Vtbl;
}
unsafe impl ::windows::core::Interface for Calendar {
    const IID: ::windows::core::GUID = <ICalendar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Calendar {
    const NAME: &'static str = "Windows.Globalization.Calendar";
}
::windows::core::interface_hierarchy!(Calendar, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Calendar {}
unsafe impl ::core::marker::Sync for Calendar {}
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct CalendarIdentifiers;
impl CalendarIdentifiers {
    pub fn Gregorian() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Gregorian)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Hebrew() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Hebrew)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Hijri() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Hijri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Japanese() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Japanese)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Julian() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Julian)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Korean() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Korean)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Taiwan() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Taiwan)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Thai() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Thai)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn UmAlQura() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UmAlQura)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Persian() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Persian)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ChineseLunar() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChineseLunar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn JapaneseLunar() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JapaneseLunar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn KoreanLunar() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KoreanLunar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TaiwanLunar() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaiwanLunar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn VietnameseLunar() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICalendarIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VietnameseLunar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICalendarIdentifiersStatics<R, F: FnOnce(&ICalendarIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CalendarIdentifiers, ICalendarIdentifiersStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICalendarIdentifiersStatics2<R, F: FnOnce(&ICalendarIdentifiersStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CalendarIdentifiers, ICalendarIdentifiersStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICalendarIdentifiersStatics3<R, F: FnOnce(&ICalendarIdentifiersStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CalendarIdentifiers, ICalendarIdentifiersStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CalendarIdentifiers {
    const NAME: &'static str = "Windows.Globalization.CalendarIdentifiers";
}
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct ClockIdentifiers;
impl ClockIdentifiers {
    pub fn TwelveHour() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IClockIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TwelveHour)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TwentyFourHour() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IClockIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TwentyFourHour)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IClockIdentifiersStatics<R, F: FnOnce(&IClockIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ClockIdentifiers, IClockIdentifiersStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ClockIdentifiers {
    const NAME: &'static str = "Windows.Globalization.ClockIdentifiers";
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
pub struct CurrencyAmount(::windows::core::IUnknown);
impl CurrencyAmount {
    pub fn Amount(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Amount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Currency(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Currency)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Create(amount: &::windows::core::HSTRING, currency: &::windows::core::HSTRING) -> ::windows::core::Result<CurrencyAmount> {
        Self::ICurrencyAmountFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(amount), ::core::mem::transmute_copy(currency), result__.as_mut_ptr()).from_abi::<CurrencyAmount>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICurrencyAmountFactory<R, F: FnOnce(&ICurrencyAmountFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CurrencyAmount, ICurrencyAmountFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CurrencyAmount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrencyAmount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrencyAmount {}
impl ::core::fmt::Debug for CurrencyAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrencyAmount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CurrencyAmount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.CurrencyAmount;{74b49942-eb75-443a-95b3-7d723f56f93c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CurrencyAmount {
    type Vtable = ICurrencyAmount_Vtbl;
}
unsafe impl ::windows::core::Interface for CurrencyAmount {
    const IID: ::windows::core::GUID = <ICurrencyAmount as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CurrencyAmount {
    const NAME: &'static str = "Windows.Globalization.CurrencyAmount";
}
::windows::core::interface_hierarchy!(CurrencyAmount, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CurrencyAmount {}
unsafe impl ::core::marker::Sync for CurrencyAmount {}
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct CurrencyIdentifiers;
impl CurrencyIdentifiers {
    pub fn AED() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AED)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AFN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AFN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ALL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ALL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AMD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AMD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ANG() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ANG)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AOA() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AOA)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ARS() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ARS)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AUD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AUD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AWG() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AWG)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AZN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AZN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BAM() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BAM)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BBD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BBD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BDT() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BDT)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BGN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BGN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BHD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BHD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BIF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BIF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BMD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BMD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BND() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BND)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BOB() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BOB)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BRL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BRL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BSD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BSD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BTN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BTN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BWP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BWP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BYR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BYR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BZD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BZD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CAD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CAD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CDF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CDF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CHF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CHF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CLP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CLP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CNY() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CNY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn COP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).COP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CRC() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CRC)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CUP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CUP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CVE() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CVE)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CZK() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CZK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DJF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DJF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DKK() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DKK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DOP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DOP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DZD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DZD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn EGP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EGP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ERN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ERN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ETB() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ETB)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn EUR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EUR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn FJD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FJD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn FKP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FKP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GBP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GBP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GEL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GEL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GHS() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GHS)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GIP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GIP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GMD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GMD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GNF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GNF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GTQ() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GTQ)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GYD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GYD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HKD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HKD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HNL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HNL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HRK() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HRK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HTG() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HTG)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HUF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HUF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IDR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IDR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ILS() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ILS)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn INR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).INR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IQD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IQD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IRR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IRR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ISK() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ISK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn JMD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JMD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn JOD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JOD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn JPY() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JPY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn KES() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KES)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn KGS() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KGS)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn KHR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KHR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn KMF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KMF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn KPW() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KPW)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn KRW() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KRW)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn KWD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KWD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn KYD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KYD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn KZT() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KZT)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LAK() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LAK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LBP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LBP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LKR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LKR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LRD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LRD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LSL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LSL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LTL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LTL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LVL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LVL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LYD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LYD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MAD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MAD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MDL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MDL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MGA() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MGA)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MKD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MKD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MMK() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MMK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MNT() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MNT)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MOP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MOP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MRO() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MRO)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MUR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MUR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MVR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MVR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MWK() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MWK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MXN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MXN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MYR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MYR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MZN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MZN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NAD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NAD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NGN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NGN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NIO() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NIO)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NOK() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NOK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NPR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NPR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NZD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NZD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn OMR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OMR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PAB() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PAB)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PEN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PEN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PGK() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PGK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PHP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PHP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PKR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PKR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PLN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PLN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PYG() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PYG)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn QAR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QAR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RON() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RON)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RSD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RSD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RUB() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RUB)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RWF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RWF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SAR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SAR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SBD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SBD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SCR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SCR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SDG() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SDG)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SEK() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SEK)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SGD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SGD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SHP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SHP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SLL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SLL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SOS() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SOS)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SRD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SRD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn STD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).STD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SYP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SYP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SZL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SZL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn THB() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).THB)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TJS() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TJS)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TMT() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TMT)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TND() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TND)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TOP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TOP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TRY() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TRY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TTD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TTD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TWD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TWD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TZS() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TZS)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn UAH() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UAH)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn UGX() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UGX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn USD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).USD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn UYU() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UYU)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn UZS() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UZS)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn VEF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VEF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn VND() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VND)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn VUV() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VUV)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn WST() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WST)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn XAF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XAF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn XCD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XCD)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn XOF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XOF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn XPF() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XPF)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn XXX() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XXX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn YER() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YER)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ZAR() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ZAR)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ZMW() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ZMW)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ZWL() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ZWL)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BYN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BYN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MRU() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MRU)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SSP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SSP)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn STN() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).STN)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn VES() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICurrencyIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VES)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICurrencyIdentifiersStatics<R, F: FnOnce(&ICurrencyIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CurrencyIdentifiers, ICurrencyIdentifiersStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrencyIdentifiersStatics2<R, F: FnOnce(&ICurrencyIdentifiersStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CurrencyIdentifiers, ICurrencyIdentifiersStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrencyIdentifiersStatics3<R, F: FnOnce(&ICurrencyIdentifiersStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CurrencyIdentifiers, ICurrencyIdentifiersStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CurrencyIdentifiers {
    const NAME: &'static str = "Windows.Globalization.CurrencyIdentifiers";
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
pub struct GeographicRegion(::windows::core::IUnknown);
impl GeographicRegion {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GeographicRegion, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Code(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CodeTwoLetter(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CodeTwoLetter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CodeThreeLetter(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CodeThreeLetter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CodeThreeDigit(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CodeThreeDigit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn NativeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NativeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CurrenciesInUse(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrenciesInUse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn CreateGeographicRegion(geographicregioncode: &::windows::core::HSTRING) -> ::windows::core::Result<GeographicRegion> {
        Self::IGeographicRegionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateGeographicRegion)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(geographicregioncode), result__.as_mut_ptr()).from_abi::<GeographicRegion>(result__)
        })
    }
    pub fn IsSupported(geographicregioncode: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IGeographicRegionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(geographicregioncode), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGeographicRegionFactory<R, F: FnOnce(&IGeographicRegionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GeographicRegion, IGeographicRegionFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGeographicRegionStatics<R, F: FnOnce(&IGeographicRegionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GeographicRegion, IGeographicRegionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for GeographicRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeographicRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeographicRegion {}
impl ::core::fmt::Debug for GeographicRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeographicRegion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GeographicRegion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.GeographicRegion;{01e9a621-4a64-4ed9-954f-9edeb07bd903})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GeographicRegion {
    type Vtable = IGeographicRegion_Vtbl;
}
unsafe impl ::windows::core::Interface for GeographicRegion {
    const IID: ::windows::core::GUID = <IGeographicRegion as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GeographicRegion {
    const NAME: &'static str = "Windows.Globalization.GeographicRegion";
}
::windows::core::interface_hierarchy!(GeographicRegion, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GeographicRegion {}
unsafe impl ::core::marker::Sync for GeographicRegion {}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
pub struct JapanesePhoneme(::windows::core::IUnknown);
impl JapanesePhoneme {
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn YomiText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).YomiText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsPhraseStart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPhraseStart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for JapanesePhoneme {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JapanesePhoneme {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JapanesePhoneme {}
impl ::core::fmt::Debug for JapanesePhoneme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JapanesePhoneme").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JapanesePhoneme {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.JapanesePhoneme;{2f6a9300-e85b-43e6-897d-5d82f862df21})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for JapanesePhoneme {
    type Vtable = IJapanesePhoneme_Vtbl;
}
unsafe impl ::windows::core::Interface for JapanesePhoneme {
    const IID: ::windows::core::GUID = <IJapanesePhoneme as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JapanesePhoneme {
    const NAME: &'static str = "Windows.Globalization.JapanesePhoneme";
}
::windows::core::interface_hierarchy!(JapanesePhoneme, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct JapanesePhoneticAnalyzer;
impl JapanesePhoneticAnalyzer {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetWords(input: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<JapanesePhoneme>> {
        Self::IJapanesePhoneticAnalyzerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetWords)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<JapanesePhoneme>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetWordsWithMonoRubyOption(input: &::windows::core::HSTRING, monoruby: bool) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<JapanesePhoneme>> {
        Self::IJapanesePhoneticAnalyzerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetWordsWithMonoRubyOption)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), monoruby, result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<JapanesePhoneme>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IJapanesePhoneticAnalyzerStatics<R, F: FnOnce(&IJapanesePhoneticAnalyzerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<JapanesePhoneticAnalyzer, IJapanesePhoneticAnalyzerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for JapanesePhoneticAnalyzer {
    const NAME: &'static str = "Windows.Globalization.JapanesePhoneticAnalyzer";
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
pub struct Language(::windows::core::IUnknown);
impl Language {
    pub fn LanguageTag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LanguageTag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn NativeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NativeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Script(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Script)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn LayoutDirection(&self) -> ::windows::core::Result<LanguageLayoutDirection> {
        let this = &::windows::core::Interface::cast::<ILanguage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LayoutDirection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<LanguageLayoutDirection>(result__)
        }
    }
    pub fn AbbreviatedName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILanguage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AbbreviatedName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetExtensionSubtags(&self, singleton: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ILanguageExtensionSubtags>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetExtensionSubtags)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(singleton), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn CreateLanguage(languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<Language> {
        Self::ILanguageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLanguage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(languagetag), result__.as_mut_ptr()).from_abi::<Language>(result__)
        })
    }
    pub fn IsWellFormed(languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::ILanguageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsWellFormed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(languagetag), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn CurrentInputMethodLanguageTag() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILanguageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentInputMethodLanguageTag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TrySetInputMethodLanguageTag(languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::ILanguageStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetInputMethodLanguageTag)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(languagetag), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMuiCompatibleLanguageListFromLanguageTags<'a, P0, E0>(languagetags: P0) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ILanguageStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMuiCompatibleLanguageListFromLanguageTags)(::windows::core::Vtable::as_raw(this), languagetags.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILanguageFactory<R, F: FnOnce(&ILanguageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Language, ILanguageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILanguageStatics<R, F: FnOnce(&ILanguageStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Language, ILanguageStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILanguageStatics2<R, F: FnOnce(&ILanguageStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Language, ILanguageStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILanguageStatics3<R, F: FnOnce(&ILanguageStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Language, ILanguageStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Language {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Language {}
impl ::core::fmt::Debug for Language {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Language").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Language {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.Language;{ea79a752-f7c2-4265-b1bd-c4dec4e4f080})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Language {
    type Vtable = ILanguage_Vtbl;
}
unsafe impl ::windows::core::Interface for Language {
    const IID: ::windows::core::GUID = <ILanguage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Language {
    const NAME: &'static str = "Windows.Globalization.Language";
}
::windows::core::interface_hierarchy!(Language, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Language {}
unsafe impl ::core::marker::Sync for Language {}
#[doc = "*Required features: `\"Globalization\"`*"]
pub struct NumeralSystemIdentifiers;
impl NumeralSystemIdentifiers {
    pub fn Arab() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Arab)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ArabExt() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ArabExt)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Bali() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bali)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Beng() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Beng)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Cham() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Cham)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Deva() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Deva)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn FullWide() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FullWide)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Gujr() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Gujr)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Guru() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Guru)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HaniDec() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HaniDec)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Java() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Java)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Kali() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kali)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Khmr() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Khmr)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Knda() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Knda)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Lana() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lana)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LanaTham() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LanaTham)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Laoo() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Laoo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Latn() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Latn)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Lepc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lepc)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Limb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Limb)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Mlym() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mlym)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Mong() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mong)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Mtei() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mtei)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Mymr() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mymr)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MymrShan() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MymrShan)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Nkoo() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Nkoo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Olck() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Olck)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Orya() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Orya)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Saur() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Saur)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sund() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Sund)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Talu() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Talu)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TamlDec() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TamlDec)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Telu() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Telu)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Thai() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Thai)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Tibt() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tibt)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Vaii() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Vaii)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Brah() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Brah)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Osma() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Osma)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MathBold() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MathBold)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MathDbl() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MathDbl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MathSans() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MathSans)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MathSanb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MathSanb)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MathMono() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MathMono)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ZmthBold() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ZmthBold)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ZmthDbl() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ZmthDbl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ZmthSans() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ZmthSans)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ZmthSanb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ZmthSanb)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ZmthMono() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::INumeralSystemIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ZmthMono)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn INumeralSystemIdentifiersStatics<R, F: FnOnce(&INumeralSystemIdentifiersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NumeralSystemIdentifiers, INumeralSystemIdentifiersStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn INumeralSystemIdentifiersStatics2<R, F: FnOnce(&INumeralSystemIdentifiersStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NumeralSystemIdentifiers, INumeralSystemIdentifiersStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for NumeralSystemIdentifiers {
    const NAME: &'static str = "Windows.Globalization.NumeralSystemIdentifiers";
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DayOfWeek(pub i32);
impl DayOfWeek {
    pub const Sunday: Self = Self(0i32);
    pub const Monday: Self = Self(1i32);
    pub const Tuesday: Self = Self(2i32);
    pub const Wednesday: Self = Self(3i32);
    pub const Thursday: Self = Self(4i32);
    pub const Friday: Self = Self(5i32);
    pub const Saturday: Self = Self(6i32);
}
impl ::core::marker::Copy for DayOfWeek {}
impl ::core::clone::Clone for DayOfWeek {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DayOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DayOfWeek {
    type Abi = Self;
}
impl ::core::fmt::Debug for DayOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DayOfWeek").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DayOfWeek {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DayOfWeek;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Globalization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LanguageLayoutDirection(pub i32);
impl LanguageLayoutDirection {
    pub const Ltr: Self = Self(0i32);
    pub const Rtl: Self = Self(1i32);
    pub const TtbLtr: Self = Self(2i32);
    pub const TtbRtl: Self = Self(3i32);
}
impl ::core::marker::Copy for LanguageLayoutDirection {}
impl ::core::clone::Clone for LanguageLayoutDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LanguageLayoutDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LanguageLayoutDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for LanguageLayoutDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LanguageLayoutDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LanguageLayoutDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.LanguageLayoutDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
