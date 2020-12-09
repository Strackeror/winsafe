const_type! { SM, i32,
	/// [`GetSystemMetrics`](crate::GetSystemMetrics) `nIndex`.

	CXSCREEN, 0
	CYSCREEN, 1
	CXVSCROLL, 2
	CYHSCROLL, 3
	CYCAPTION, 4
	CXBORDER, 5
	CYBORDER, 6
	CXDLGFRAME, 7
	CYDLGFRAME, 8
	CYVTHUMB, 9
	CXHTHUMB, 10
	CXICON, 11
	CYICON, 12
	CXCURSOR, 13
	CYCURSOR, 14
	CYMENU, 15
	CXFULLSCREEN, 16
	CYFULLSCREEN, 17
	CYKANJIWINDOW, 18
	MOUSEPRESENT, 19
	CYVSCROLL, 20
	CXHSCROLL, 21
	DEBUG, 22
	SWAPBUTTON, 23
	RESERVED1, 24
	RESERVED2, 25
	RESERVED3, 26
	RESERVED4, 27
	CXMIN, 28
	CYMIN, 29
	CXSIZE, 30
	CYSIZE, 31
	CXFRAME, 32
	CYFRAME, 33
	CXMINTRACK, 34
	CYMINTRACK, 35
	CXDOUBLECLK, 36
	CYDOUBLECLK, 37
	CXICONSPACING, 38
	CYICONSPACING, 39
	MENUDROPALIGNMENT, 40
	PENWINDOWS, 41
	DBCSENABLED, 42
	CMOUSEBUTTONS, 43
	CXFIXEDFRAME, Self::CXDLGFRAME.0
	CYFIXEDFRAME, Self::CYDLGFRAME.0
	CXSIZEFRAME, Self::CXFRAME.0
	CYSIZEFRAME, Self::CYFRAME.0
	SECURE, 44
	CXEDGE, 45
	CYEDGE, 46
	CXMINSPACING, 47
	CYMINSPACING, 48
	CXSMICON, 49
	CYSMICON, 50
	CYSMCAPTION, 51
	CXSMSIZE, 52
	CYSMSIZE, 53
	CXMENUSIZE, 54
	CYMENUSIZE, 55
	ARRANGE, 56
	CXMINIMIZED, 57
	CYMINIMIZED, 58
	CXMAXTRACK, 59
	CYMAXTRACK, 60
	CXMAXIMIZED, 61
	CYMAXIMIZED, 62
	NETWORK, 63
	CLEANBOOT, 67
	CXDRAG, 68
	CYDRAG, 69
	SHOWSOUNDS, 70
	CXMENUCHECK, 71
	CYMENUCHECK, 72
	SLOWMACHINE, 73
	MIDEASTENABLED, 74
	MOUSEWHEELPRESENT, 75
	XVIRTUALSCREEN, 76
	YVIRTUALSCREEN, 77
	CXVIRTUALSCREEN, 78
	CYVIRTUALSCREEN, 79
	CMONITORS, 80
	SAMEDISPLAYFORMAT, 81
	IMMENABLED, 82
	CXFOCUSBORDER, 83
	CYFOCUSBORDER, 84
	TABLETPC, 86
	MEDIACENTER, 87
	STARTER, 88
	SERVERR2, 89
	MOUSEHORIZONTALWHEELPRESENT, 91
	CXPADDEDBORDER, 92
	DIGITIZER, 94
	MAXIMUMTOUCHES, 95
	CMETRICS, 97
	REMOTESESSION, 0x1000
	SHUTTINGDOWN, 0x2000
	REMOTECONTROL, 0x2001
	CARETBLINKINGENABLED, 0x2002
	CONVERTIBLESLATEMODE, 0x2003
	SYSTEMDOCKED, 0x2004
}

const_type! { SUBLANG, u16,
	/// [`FormatMessage`](crate::co::ERROR::FormatMessage) `dwLanguageId`, used
	/// with [`LANG`](crate::co::LANG).

	NEUTRAL, 0x00
	DEFAULT, 0x01
	SYS_DEFAULT, 0x02
	CUSTOM_DEFAULT, 0x03
	CUSTOM_UNSPECIFIED, 0x04
	UI_CUSTOM_DEFAULT, 0x05
	AFRIKAANS_SOUTH_AFRICA, 0x01
	ALBANIAN_ALBANIA, 0x01
	ALSATIAN_FRANCE, 0x01
	AMHARIC_ETHIOPIA, 0x01
	ARABIC_SAUDI_ARABIA, 0x01
	ARABIC_IRAQ, 0x02
	ARABIC_EGYPT, 0x03
	ARABIC_LIBYA, 0x04
	ARABIC_ALGERIA, 0x05
	ARABIC_MOROCCO, 0x06
	ARABIC_TUNISIA, 0x07
	ARABIC_OMAN, 0x08
	ARABIC_YEMEN, 0x09
	ARABIC_SYRIA, 0x0a
	ARABIC_JORDAN, 0x0b
	ARABIC_LEBANON, 0x0c
	ARABIC_KUWAIT, 0x0d
	ARABIC_UAE, 0x0e
	ARABIC_BAHRAIN, 0x0f
	ARABIC_QATAR, 0x10
	ARMENIAN_ARMENIA, 0x01
	ASSAMESE_INDIA, 0x01
	AZERI_LATIN, 0x01
	AZERI_CYRILLIC, 0x02
	AZERBAIJANI_AZERBAIJAN_LATIN, 0x01
	AZERBAIJANI_AZERBAIJAN_CYRILLIC, 0x02
	BANGLA_INDIA, 0x01
	BANGLA_BANGLADESH, 0x02
	BASHKIR_RUSSIA, 0x01
	BASQUE_BASQUE, 0x01
	BELARUSIAN_BELARUS, 0x01
	BENGALI_INDIA, 0x01
	BENGALI_BANGLADESH, 0x02
	BOSNIAN_BOSNIA_HERZEGOVINA_LATIN, 0x05
	BOSNIAN_BOSNIA_HERZEGOVINA_CYRILLIC, 0x08
	BRETON_FRANCE, 0x01
	BULGARIAN_BULGARIA, 0x01
	CATALAN_CATALAN, 0x01
	CENTRAL_KURDISH_IRAQ, 0x01
	CHEROKEE_CHEROKEE, 0x01
	CHINESE_TRADITIONAL, 0x01
	CHINESE_SIMPLIFIED, 0x02
	CHINESE_HONGKONG, 0x03
	CHINESE_SINGAPORE, 0x04
	CHINESE_MACAU, 0x05
	CORSICAN_FRANCE, 0x01
	CZECH_CZECH_REPUBLIC, 0x01
	CROATIAN_CROATIA, 0x01
	CROATIAN_BOSNIA_HERZEGOVINA_LATIN, 0x04
	DANISH_DENMARK, 0x01
	DARI_AFGHANISTAN, 0x01
	DIVEHI_MALDIVES, 0x01
	DUTCH, 0x01
	DUTCH_BELGIAN, 0x02
	ENGLISH_US, 0x01
	ENGLISH_UK, 0x02
	ENGLISH_AUS, 0x03
	ENGLISH_CAN, 0x04
	ENGLISH_NZ, 0x05
	ENGLISH_EIRE, 0x06
	ENGLISH_SOUTH_AFRICA, 0x07
	ENGLISH_JAMAICA, 0x08
	ENGLISH_CARIBBEAN, 0x09
	ENGLISH_BELIZE, 0x0a
	ENGLISH_TRINIDAD, 0x0b
	ENGLISH_ZIMBABWE, 0x0c
	ENGLISH_PHILIPPINES, 0x0d
	ENGLISH_INDIA, 0x10
	ENGLISH_MALAYSIA, 0x11
	ENGLISH_SINGAPORE, 0x12
	ESTONIAN_ESTONIA, 0x01
	FAEROESE_FAROE_ISLANDS, 0x01
	FILIPINO_PHILIPPINES, 0x01
	FINNISH_FINLAND, 0x01
	FRENCH, 0x01
	FRENCH_BELGIAN, 0x02
	FRENCH_CANADIAN, 0x03
	FRENCH_SWISS, 0x04
	FRENCH_LUXEMBOURG, 0x05
	FRENCH_MONACO, 0x06
	FRISIAN_NETHERLANDS, 0x01
	FULAH_SENEGAL, 0x02
	GALICIAN_GALICIAN, 0x01
	GEORGIAN_GEORGIA, 0x01
	GERMAN, 0x01
	GERMAN_SWISS, 0x02
	GERMAN_AUSTRIAN, 0x03
	GERMAN_LUXEMBOURG, 0x04
	GERMAN_LIECHTENSTEIN, 0x05
	GREEK_GREECE, 0x01
	GREENLANDIC_GREENLAND, 0x01
	GUJARATI_INDIA, 0x01
	HAUSA_NIGERIA_LATIN, 0x01
	HAWAIIAN_US, 0x01
	HEBREW_ISRAEL, 0x01
	HINDI_INDIA, 0x01
	HUNGARIAN_HUNGARY, 0x01
	ICELANDIC_ICELAND, 0x01
	IGBO_NIGERIA, 0x01
	INDONESIAN_INDONESIA, 0x01
	INUKTITUT_CANADA, 0x01
	INUKTITUT_CANADA_LATIN, 0x02
	IRISH_IRELAND, 0x02
	ITALIAN, 0x01
	ITALIAN_SWISS, 0x02
	JAPANESE_JAPAN, 0x01
	KANNADA_INDIA, 0x01
	KASHMIRI_SASIA, 0x02
	KASHMIRI_INDIA, 0x02
	KAZAK_KAZAKHSTAN, 0x01
	KHMER_CAMBODIA, 0x01
	KICHE_GUATEMALA, 0x01
	KINYARWANDA_RWANDA, 0x01
	KONKANI_INDIA, 0x01
	KOREAN, 0x01
	KYRGYZ_KYRGYZSTAN, 0x01
	LAO_LAO, 0x01
	LATVIAN_LATVIA, 0x01
	LITHUANIAN, 0x01
	LOWER_SORBIAN_GERMANY, 0x02
	LUXEMBOURGISH_LUXEMBOURG, 0x01
	MACEDONIAN_MACEDONIA, 0x01
	MALAY_MALAYSIA, 0x01
	MALAY_BRUNEI_DARUSSALAM, 0x02
	MALAYALAM_INDIA, 0x01
	MALTESE_MALTA, 0x01
	MAORI_NEW_ZEALAND, 0x01
	MAPUDUNGUN_CHILE, 0x01
	MARATHI_INDIA, 0x01
	MOHAWK_MOHAWK, 0x01
	MONGOLIAN_CYRILLIC_MONGOLIA, 0x01
	MONGOLIAN_PRC, 0x02
	NEPALI_INDIA, 0x02
	NEPALI_NEPAL, 0x01
	NORWEGIAN_BOKMAL, 0x01
	NORWEGIAN_NYNORSK, 0x02
	OCCITAN_FRANCE, 0x01
	ODIA_INDIA, 0x01
	ORIYA_INDIA, 0x01
	PASHTO_AFGHANISTAN, 0x01
	PERSIAN_IRAN, 0x01
	POLISH_POLAND, 0x01
	PORTUGUESE, 0x02
	PORTUGUESE_BRAZILIAN, 0x01
	PULAR_SENEGAL, 0x02
	PUNJABI_INDIA, 0x01
	PUNJABI_PAKISTAN, 0x02
	QUECHUA_BOLIVIA, 0x01
	QUECHUA_ECUADOR, 0x02
	QUECHUA_PERU, 0x03
	ROMANIAN_ROMANIA, 0x01
	ROMANSH_SWITZERLAND, 0x01
	RUSSIAN_RUSSIA, 0x01
	SAKHA_RUSSIA, 0x01
	SAMI_NORTHERN_NORWAY, 0x01
	SAMI_NORTHERN_SWEDEN, 0x02
	SAMI_NORTHERN_FINLAND, 0x03
	SAMI_LULE_NORWAY, 0x04
	SAMI_LULE_SWEDEN, 0x05
	SAMI_SOUTHERN_NORWAY, 0x06
	SAMI_SOUTHERN_SWEDEN, 0x07
	SAMI_SKOLT_FINLAND, 0x08
	SAMI_INARI_FINLAND, 0x09
	SANSKRIT_INDIA, 0x01
	SCOTTISH_GAELIC, 0x01
	SERBIAN_BOSNIA_HERZEGOVINA_LATIN, 0x06
	SERBIAN_BOSNIA_HERZEGOVINA_CYRILLIC, 0x07
	SERBIAN_MONTENEGRO_LATIN, 0x0b
	SERBIAN_MONTENEGRO_CYRILLIC, 0x0c
	SERBIAN_SERBIA_LATIN, 0x09
	SERBIAN_SERBIA_CYRILLIC, 0x0a
	SERBIAN_CROATIA, 0x01
	SERBIAN_LATIN, 0x02
	SERBIAN_CYRILLIC, 0x03
	SINDHI_INDIA, 0x01
	SINDHI_PAKISTAN, 0x02
	SINDHI_AFGHANISTAN, 0x02
	SINHALESE_SRI_LANKA, 0x01
	SOTHO_NORTHERN_SOUTH_AFRICA, 0x01
	SLOVAK_SLOVAKIA, 0x01
	SLOVENIAN_SLOVENIA, 0x01
	SPANISH, 0x01
	SPANISH_MEXICAN, 0x02
	SPANISH_MODERN, 0x03
	SPANISH_GUATEMALA, 0x04
	SPANISH_COSTA_RICA, 0x05
	SPANISH_PANAMA, 0x06
	SPANISH_DOMINICAN_REPUBLIC, 0x07
	SPANISH_VENEZUELA, 0x08
	SPANISH_COLOMBIA, 0x09
	SPANISH_PERU, 0x0a
	SPANISH_ARGENTINA, 0x0b
	SPANISH_ECUADOR, 0x0c
	SPANISH_CHILE, 0x0d
	SPANISH_URUGUAY, 0x0e
	SPANISH_PARAGUAY, 0x0f
	SPANISH_BOLIVIA, 0x10
	SPANISH_EL_SALVADOR, 0x11
	SPANISH_HONDURAS, 0x12
	SPANISH_NICARAGUA, 0x13
	SPANISH_PUERTO_RICO, 0x14
	SPANISH_US, 0x15
	SWAHILI_KENYA, 0x01
	SWEDISH, 0x01
	SWEDISH_FINLAND, 0x02
	SYRIAC_SYRIA, 0x01
	TAJIK_TAJIKISTAN, 0x01
	TAMAZIGHT_ALGERIA_LATIN, 0x02
	TAMAZIGHT_MOROCCO_TIFINAGH, 0x04
	TAMIL_INDIA, 0x01
	TAMIL_SRI_LANKA, 0x02
	TATAR_RUSSIA, 0x01
	TELUGU_INDIA, 0x01
	THAI_THAILAND, 0x01
	TIBETAN_PRC, 0x01
	TIGRIGNA_ERITREA, 0x02
	TIGRINYA_ERITREA, 0x02
	TIGRINYA_ETHIOPIA, 0x01
	TSWANA_BOTSWANA, 0x02
	TSWANA_SOUTH_AFRICA, 0x01
	TURKISH_TURKEY, 0x01
	TURKMEN_TURKMENISTAN, 0x01
	UIGHUR_PRC, 0x01
	UKRAINIAN_UKRAINE, 0x01
	UPPER_SORBIAN_GERMANY, 0x01
	URDU_PAKISTAN, 0x01
	URDU_INDIA, 0x02
	UZBEK_LATIN, 0x01
	UZBEK_CYRILLIC, 0x02
	VALENCIAN_VALENCIA, 0x02
	VIETNAMESE_VIETNAM, 0x01
	WELSH_UNITED_KINGDOM, 0x01
	WOLOF_SENEGAL, 0x01
	XHOSA_SOUTH_AFRICA, 0x01
	YAKUT_RUSSIA, 0x01
	YI_PRC, 0x01
	YORUBA_NIGERIA, 0x01
	ZULU_SOUTH_AFRICA, 0x01
}

const_type! { SW, i32,
	/// [`ShowWindow`](crate::HWND::ShowWindow) `nCmdShow`.

	HIDE, 0
	SHOWNORMAL, 1
	NORMAL, 1
	SHOWMINIMIZED, 2
	SHOWMAXIMIZED, 3
	MAXIMIZE, 3
	SHOWNOACTIVATE, 4
	SHOW, 5
	MINIMIZE, 6
	SHOWMINNOACTIVE, 7
	SHOWNA, 8
	RESTORE, 9
	SHOWDEFAULT, 10
	FORCEMINIMIZE, 11
	MAX, 11
}