var searchIndex = JSON.parse('{\
"triseratops":{"doc":"This library provides access to metadata and libraries…","i":[[0,"error","triseratops","Error types and helper functions.",null,null],[4,"Error","triseratops::error","Error enumerates all possible errors returned by this…",null,null],[13,"ParseError","","Represents an generic parse error.",0,null],[13,"UnsupportedTagFormat","","Thrown when trying to parse a tag with an unsupported tag…",0,null],[13,"ParseIncomplete","","Represents an generic parse error.",0,null],[13,"VerboseParseError","","Represents an generic parse error.",0,null],[12,"errors","triseratops::error::Error","",1,null],[13,"Base64DecodeError","triseratops::error","Represents decode error.",0,null],[12,"source","triseratops::error::Error","",2,null],[13,"Base64InvalidLengthError","triseratops::error","Represents decode error.",0,null],[12,"length","triseratops::error::Error","",3,null],[13,"EnvelopeParseError","triseratops::error","Represents decode error.",0,null],[13,"EnvelopeNameMismatch","","Represents decode error.",0,null],[12,"expected","triseratops::error::Error","",4,null],[12,"actual","","",4,null],[13,"NoTagDataAvailable","triseratops::error","Happens when trying to write tag data even though no data…",0,null],[13,"IOError","","Represents all other cases of `std::io::Error`.",0,null],[0,"library","triseratops","Parsers for the Serato library database and crates",null,null],[0,"database","triseratops::library","",null,null],[4,"Field","triseratops::library::database","",null,null],[13,"Unknown","","",5,null],[12,"field_type","triseratops::library::database::Field","",6,null],[12,"name","","",6,null],[12,"content","","",6,null],[13,"UnknownBoolean","triseratops::library::database","",5,null],[12,"name","triseratops::library::database::Field","",7,null],[12,"value","","",7,null],[13,"UnknownContainerField","triseratops::library::database","",5,null],[12,"name","triseratops::library::database::Field","",8,null],[12,"fields","","",8,null],[13,"UnknownContainerRField","triseratops::library::database","",5,null],[12,"name","triseratops::library::database::Field","",9,null],[12,"fields","","",9,null],[13,"UnknownPathField","triseratops::library::database","",5,null],[12,"name","triseratops::library::database::Field","",10,null],[12,"path","","",10,null],[13,"UnknownU16Field","triseratops::library::database","",5,null],[12,"name","triseratops::library::database::Field","",11,null],[12,"value","","",11,null],[13,"UnknownU32Field","triseratops::library::database","",5,null],[12,"name","triseratops::library::database::Field","",12,null],[12,"value","","",12,null],[13,"UnknownTextField","triseratops::library::database","",5,null],[12,"name","triseratops::library::database::Field","",13,null],[12,"text","","",13,null],[13,"Album","triseratops::library::database","",5,null],[13,"Artist","","",5,null],[13,"BPM","","",5,null],[13,"BeatgridLocked","","",5,null],[13,"Bitrate","","",5,null],[13,"Comment","","",5,null],[13,"Composer","","",5,null],[13,"DateAdded","","",5,null],[13,"DateAddedStr","","",5,null],[13,"FilePath","","",5,null],[13,"FileSize","","",5,null],[13,"FileTime","","",5,null],[13,"FileType","","",5,null],[13,"Genre","","",5,null],[13,"Grouping","","",5,null],[13,"Key","","",5,null],[13,"Label","","",5,null],[13,"Length","","",5,null],[13,"Missing","","",5,null],[13,"SampleRate","","",5,null],[13,"SongTitle","","",5,null],[13,"Track","","",5,null],[13,"Version","","",5,null],[13,"Year","","",5,null],[13,"Sorting","","",5,null],[13,"ReverseOrder","","",5,null],[13,"ColumnTitle","","",5,null],[13,"ColumnName","","",5,null],[13,"ColumnWidth","","",5,null],[13,"TrackPath","","",5,null],[5,"parse","","",null,[[],[["result",4],["vec",3],["error",4]]]],[6,"Path","","",null,null],[0,"filesystem","triseratops::library","",null,null],[3,"SeratoLibraryInfo","triseratops::library::filesystem","",null,null],[12,"path","","",14,null],[12,"database_path","","",14,null],[12,"crates","","",14,null],[5,"get_library","","",null,[[["pathbuf",3]],[["seratolibraryinfo",3],["option",4]]]],[6,"CrateInfo","","",null,null],[0,"tag","triseratops","Parsers for Serato\'s file tags",null,null],[0,"format","triseratops::tag","Traits for the different tag types (ID3, FLAC, etc.)",null,null],[0,"enveloped","triseratops::tag::format","Helper for FLAC and MP4 tags",null,null],[5,"parse_envelope","triseratops::tag::format::enveloped","",null,[[],[["error",4],["result",4]]]],[5,"is_base64","","",null,[[]]],[5,"is_newline","","",null,[[]]],[5,"take_base64_with_newline","","",null,[[],[["iresult",6],["verboseerror",3]]]],[5,"base64_decode","","",null,[[],[["error",4],["result",4],["vec",3]]]],[5,"base64_encode","","",null,[[],[["result",4],["error",4]]]],[5,"envelope_decode","","",null,[[],[["error",4],["result",4]]]],[5,"envelope_decode_with_name","","",null,[[],[["error",4],["result",4],["vec",3]]]],[5,"envelope_encode_with_name","","",null,[[],[["result",4],["error",4]]]],[8,"EnvelopedTag","","",null,null],[11,"parse_enveloped","","",15,[[],[["result",4],["error",4]]]],[11,"write_enveloped","","",15,[[],[["result",4],["error",4]]]],[0,"flac","triseratops::tag::format","FLAC tags",null,null],[8,"FLACTag","triseratops::tag::format::flac","",null,null],[18,"FLAC_COMMENT","","Name of the `VORBIS_COMMENT` that this data is stored in.",16,null],[11,"parse_flac","","",16,[[],[["result",4],["error",4]]]],[11,"write_flac","","",16,[[],[["result",4],["error",4]]]],[0,"id3","triseratops::tag::format","ID3 tags",null,null],[8,"ID3Tag","triseratops::tag::format::id3","",null,null],[18,"ID3_TAG","","Name of the ID3 tag that this data is stored in.",17,null],[11,"parse_id3","","",17,[[],[["result",4],["error",4]]]],[11,"write_id3","","",17,[[],[["result",4],["error",4]]]],[0,"mp4","triseratops::tag::format","MP4 tags",null,null],[8,"MP4Tag","triseratops::tag::format::mp4","",null,null],[18,"MP4_ATOM_FREEFORM_MEAN","","The mean part of the freeform `MP4_ATOM` that this data is…",18,null],[18,"MP4_ATOM_FREEFORM_NAME","","The mean part of the freeform `MP4_ATOM` that this data is…",18,null],[11,"parse_mp4","","",18,[[],[["result",4],["error",4]]]],[11,"write_mp4","","",18,[[],[["result",4],["error",4]]]],[0,"ogg","triseratops::tag::format","Ogg tags",null,null],[8,"OggTag","triseratops::tag::format::ogg","",null,null],[18,"OGG_COMMENT","","Name of the `MP4_ATOM` that this data is stored in.",19,null],[10,"parse_ogg","","",19,[[],[["result",4],["error",4]]]],[10,"write_ogg","","",19,[[],[["result",4],["error",4]]]],[8,"Tag","triseratops::tag::format","",null,null],[18,"NAME","","",20,null],[10,"parse","","",20,[[],[["result",4],["error",4]]]],[10,"write","","",20,[[],[["result",4],["error",4]]]],[0,"generic","triseratops::tag","Generic structs",null,null],[3,"Color","triseratops::tag::generic","Represents a 3-Byte RGB color value.",null,null],[12,"red","","",21,null],[12,"green","","",21,null],[12,"blue","","",21,null],[3,"Version","","Represents 2-Byte version value.",null,null],[12,"major","","",22,null],[12,"minor","","",22,null],[3,"Cue","","A cue point.",null,null],[12,"index","","",23,null],[12,"position_millis","","",23,null],[12,"color","","",23,null],[12,"label","","",23,null],[3,"Loop","","A saved loops.",null,null],[12,"index","","",24,null],[12,"start_position_millis","","",24,null],[12,"end_position_millis","","",24,null],[12,"color","","",24,null],[12,"is_locked","","",24,null],[12,"label","","",24,null],[3,"Flip","","A Serato Flip performances.",null,null],[12,"index","","",25,null],[12,"is_enabled","","",25,null],[12,"label","","",25,null],[12,"is_loop","","",25,null],[12,"actions","","",25,null],[3,"CensorFlipAction","","A \\\"Censor\\\" action inside of a `Flip` performance.",null,null],[12,"start_position_seconds","","The start position of the censoring.",26,null],[12,"end_position_seconds","","The end position of the censoring.",26,null],[12,"speed_factor","","The playback speed factor (usually -1.0).",26,null],[3,"JumpFlipAction","","A \\\"Jump\\\" action inside of a `Flip` performance.",null,null],[12,"source_position_seconds","","The source position of the jump.",27,null],[12,"target_position_seconds","","The target position of the jump.",27,null],[3,"UnknownFlipAction","","A unknown action inside of a `Flip` performance that we…",null,null],[12,"id","","",28,null],[12,"data","","",28,null],[4,"FlipAction","","An action inside of a `Flip` performance.",null,null],[13,"Censor","","",29,null],[13,"Jump","","",29,null],[13,"Unknown","","",29,null],[0,"serato32","triseratops::tag","These functions convert between a custom 4-byte format…",null,null],[5,"decode","triseratops::tag::serato32","Decodes value from Serato\'s 32-bit custom format to 24-bit…",null,[[]]],[5,"encode","","Encodes 3-byte value to to Serato\'s 32-bit custom format.",null,[[]]],[5,"take","","Returns a 3-byte tuple decoded from the first 4 input bytes.",null,[[],[["iresult",6],["verboseerror",3]]]],[5,"write","","",null,[[],[["result",4],["error",4]]]],[5,"take_color","","Returns a `Color` decoded from the first 4 input bytes.",null,[[],[["color",3],["iresult",6],["verboseerror",3]]]],[5,"write_color","","",null,[[["color",3]],[["result",4],["error",4]]]],[5,"take_u32","","Returns a `u32` decoded from the first 4 input bytes.",null,[[],[["iresult",6],["verboseerror",3]]]],[5,"write_u32","","",null,[[],[["result",4],["error",4]]]],[0,"container","triseratops::tag","Provides the `TagContainer` struct, a simple interface to…",null,null],[3,"TagContainer","triseratops::tag::container","Provides a streamlined interface for retrieving Serato tag…",null,null],[4,"TagFormat","","The tag type of the data.",null,null],[13,"ID3","","",30,null],[13,"FLAC","","",30,null],[13,"MP4","","",30,null],[13,"Ogg","","",30,null],[11,"new","","Create an empty Serato tag container.",31,[[]]],[11,"parse_autotags","","Parse the `Serato Autotags` tag.",31,[[["tagformat",4]],[["result",4],["error",4]]]],[11,"write_autotags","","Write the `Serato Autotags` tag.",31,[[["tagformat",4]],[["result",4],["error",4]]]],[11,"parse_beatgrid","","Parse the `Serato BeatGrid` tag.",31,[[["tagformat",4]],[["result",4],["error",4]]]],[11,"write_beatgrid","","Write the `Serato BeatGrid` tag.",31,[[["tagformat",4]],[["result",4],["error",4]]]],[11,"parse_markers","","Parse the `Serato Markers_` tag.",31,[[["tagformat",4]],[["result",4],["error",4]]]],[11,"write_markers","","Write the `Serato Markers_` tag.",31,[[["tagformat",4]],[["result",4],["error",4]]]],[11,"parse_markers2","","Parse the `Serato Markers2` tag.",31,[[["tagformat",4]],[["result",4],["error",4]]]],[11,"write_markers2","","Write the `Serato Markers2` tag.",31,[[["tagformat",4]],[["result",4],["error",4]]]],[11,"parse_overview","","Parse the `Serato Overview` tag.",31,[[["tagformat",4]],[["result",4],["error",4]]]],[11,"write_overview","","Write the `Serato Overview` tag.",31,[[["tagformat",4]],[["result",4],["error",4]]]],[11,"auto_gain","","Returns the `auto_gain` value from the `Serato Autotags`…",31,[[],["option",4]]],[11,"gain_db","","Returns the `gain_db` value from the `Serato Autotags` tag.",31,[[],["option",4]]],[11,"beatgrid","","Returns the beatgrid from the `Serato BeatGrid` tag.",31,[[],["option",4]]],[11,"bpm_locked","","Returns BPM lock status from the `Serato Markers2` tag.",31,[[],["option",4]]],[11,"cues","","Returns cues from the `Serato Markers_` and `Serato…",31,[[],[["cue",3],["vec",3]]]],[11,"loops","","Returns loops from the `Serato Markers_` and `Serato…",31,[[],[["loop",3],["vec",3]]]],[11,"flips","","Returns flips from the `Serato Markers2` tag.",31,[[],[["flip",3],["vec",3]]]],[11,"track_color","","Returns the track color from the `Serato Markers_` and…",31,[[],[["option",4],["color",3]]]],[11,"overview","","Returns the waveform overview data color from the `Serato…",31,[[],[["vec",3],["option",4]]]],[0,"analysis","triseratops::tag","The `Serato Analysis` tag stores the analysis version.",null,null],[3,"Analysis","triseratops::tag::analysis","Represents the  `Serato Analysis` tag.",null,null],[12,"version","","The analysis version.",32,null],[5,"parse_analysis","","",null,[[],[["result",4],["analysis",3],["error",4]]]],[5,"parse_analysis_ogg","","",null,[[],[["result",4],["analysis",3],["error",4]]]],[5,"write_analysis","","Serialize `Analysis` struct to bytes.",null,[[["analysis",3]],[["result",4],["error",4]]]],[5,"write_analysis_ogg","","Serialize `Analysis` struct to bytes (Ogg version).",null,[[["analysis",3]],[["result",4],["error",4]]]],[0,"autotags","triseratops::tag","The `Serato Autotags` tag stores BPM and Gain values.",null,null],[3,"Autotags","triseratops::tag::autotags","Represents the  `Serato AutoTags` tag.",null,null],[12,"version","","The tag version.",33,null],[12,"bpm","","The track\'s number of beats per minute (BPM).",33,null],[12,"auto_gain","","The track\'s autogain values (probably comparable to…",33,null],[12,"gain_db","","The track\'s gain value (manual?).",33,null],[5,"write_double_str","","",null,[[],[["result",4],["error",4]]]],[5,"write_autotags","","",null,[[["autotags",3]],[["result",4],["error",4]]]],[0,"beatgrid","triseratops::tag","The `Serato BeatGrid` tag stores the beatgrid markers.",null,null],[3,"TerminalMarker","triseratops::tag::beatgrid","Represents the terminal beatgrid marker in the `Serato…",null,null],[12,"position","","The position in seconds.",34,null],[12,"bpm","","The track\'s beats per minute (BPM).",34,null],[3,"NonTerminalMarker","","Represents a non-terminal beatgrid marker in the `Serato…",null,null],[12,"position","","The position in seconds.",35,null],[12,"beats_till_next_marker","","The number of beats between this marker and the next one…",35,null],[3,"Beatgrid","","Represents the `Serato BeatGrid` tag.",null,null],[12,"version","","The analysis version.",36,null],[12,"non_terminal_markers","","Zero or more non-terminal beatgrid markers.",36,null],[12,"terminal_marker","","The terminal beatgrid marker.",36,null],[12,"footer","","A single footer byte that is apparently random (?).",36,null],[5,"write_non_terminal_marker","","",null,[[["nonterminalmarker",3]],[["result",4],["error",4]]]],[5,"write_terminal_marker","","",null,[[["terminalmarker",3]],[["result",4],["error",4]]]],[5,"write_beatgrid","","",null,[[["beatgrid",3]],[["result",4],["error",4]]]],[0,"markers","triseratops::tag","The `Serato Markers_` tag stores information about the…",null,null],[3,"Marker","triseratops::tag::markers","Represents a single marker in the `Serato Markers_` tag.",null,null],[12,"start_position_millis","","The position of the loop or cue.",37,null],[12,"end_position_millis","","If this is a loop, this field stores the end position.",37,null],[12,"color","","The color of the cue.",37,null],[12,"marker_type","","The type of this marker.",37,null],[12,"is_locked","","Indicates whether the loop is locked.",37,null],[3,"Markers","","Represents the `Serato Markers_` tag.",null,null],[12,"version","","The tag version.",38,null],[12,"entries","","The marker entries.",38,null],[12,"track_color","","The color of the track in Serato\'s library view.",38,null],[4,"MarkerType","","Type of a Marker.",null,null],[13,"Invalid","","Used for unset cues.",39,null],[13,"Cue","","Used for set cues.",39,null],[13,"Loop","","Used for loops (both set and unset ones).",39,null],[5,"take_bool","","Returns a bool parsed from the next input byte.",null,[[],[["iresult",6],["verboseerror",3]]]],[5,"take_position","","Returns an `Option<u32>` which contains the position…",null,[[],[["option",4],["iresult",6],["verboseerror",3]]]],[5,"write_markers","","",null,[[["markers",3]],[["result",4],["error",4]]]],[5,"write_markers_mp4","","",null,[[["markers",3]],[["result",4],["error",4]]]],[11,"cues","","",38,[[],["vec",3]]],[11,"loops","","",38,[[],["vec",3]]],[11,"track_color","","",38,[[],["color",3]]],[0,"markers2","triseratops::tag","The `Serato Markers2` tag stores various kinds of track…",null,null],[3,"UnknownMarker","triseratops::tag::markers2","An unknown marker that we don\'t have a parser for.",null,null],[12,"name","","",40,null],[12,"data","","",40,null],[3,"TrackColorMarker","","A `COLOR` marker.",null,null],[12,"color","","",41,null],[3,"BPMLockMarker","","A `BPMLOCK` marker.",null,null],[12,"is_locked","","",42,null],[3,"Markers2","","Represents the `Serato Markers2` tag.",null,null],[12,"version","","",43,null],[12,"size","","",43,null],[12,"content","","",43,null],[3,"Markers2Content","","Represents the base64-encoded content of the `Serato…",null,null],[12,"version","","",44,null],[12,"markers","","",44,null],[4,"Marker","","A marker in the `Serato Markers2` tag.",null,null],[13,"Unknown","","",45,null],[13,"Color","","",45,null],[13,"BPMLock","","",45,null],[13,"Cue","","",45,null],[13,"Loop","","",45,null],[13,"Flip","","",45,null],[5,"peek_nullbyte","","Returns a nullbyte from the input slice (non-consuming).",null,[[],[["iresult",6],["verboseerror",3]]]],[5,"peek_newline_or_nullbyte","","Returns a nullbyte or newline character from the input…",null,[[],[["iresult",6],["verboseerror",3]]]],[11,"bpm_locked","","",43,[[],["option",4]]],[11,"cues","","",43,[[],[["vec",3],["cue",3]]]],[11,"loops","","",43,[[],[["vec",3],["loop",3]]]],[11,"flips","","",43,[[],[["vec",3],["flip",3]]]],[11,"track_color","","",43,[[],[["color",3],["option",4]]]],[0,"overview","triseratops::tag","The `Serato Overview` tag stores the waveform overview data.",null,null],[3,"Overview","triseratops::tag::overview","Represents the `Serato Overview` tag.",null,null],[12,"version","","The tag version.",46,null],[12,"data","","The Waveform overview data.",46,null],[5,"write_overview","","",null,[[["overview",3]],[["result",4],["error",4]]]],[0,"relvolad","triseratops::tag","The `Serato RelVolAd` tag stores the analysis version.",null,null],[3,"RelVolAd","triseratops::tag::relvolad","Represents the  `Serato RelVolAd` tag.",null,null],[12,"version","","The `RelVolAd` version.",47,null],[12,"data","","The data (not reverse-engineered yet)",47,null],[0,"vidassoc","triseratops::tag","The `Serato VidAssoc` tag stores the analysis version.",null,null],[3,"VidAssoc","triseratops::tag::vidassoc","Represents the  `Serato VidAssoc` tag.",null,null],[12,"version","","The `VidAssoc` version.",48,null],[12,"data","","The data (not reverse-engineered yet)",48,null],[11,"from","triseratops::error","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_string","","",0,[[],["string",3]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","triseratops::library::database","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","triseratops::library::filesystem","",14,[[]]],[11,"into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","triseratops::tag::generic","",21,[[]]],[11,"into","","",21,[[]]],[11,"to_owned","","",21,[[]]],[11,"clone_into","","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"borrow_mut","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"try_into","","",21,[[],["result",4]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"from","","",22,[[]]],[11,"into","","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"borrow_mut","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"try_into","","",22,[[],["result",4]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"from","","",23,[[]]],[11,"into","","",23,[[]]],[11,"to_owned","","",23,[[]]],[11,"clone_into","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"borrow_mut","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"try_into","","",23,[[],["result",4]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"from","","",24,[[]]],[11,"into","","",24,[[]]],[11,"to_owned","","",24,[[]]],[11,"clone_into","","",24,[[]]],[11,"borrow","","",24,[[]]],[11,"borrow_mut","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"try_into","","",24,[[],["result",4]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"from","","",25,[[]]],[11,"into","","",25,[[]]],[11,"to_owned","","",25,[[]]],[11,"clone_into","","",25,[[]]],[11,"borrow","","",25,[[]]],[11,"borrow_mut","","",25,[[]]],[11,"try_from","","",25,[[],["result",4]]],[11,"try_into","","",25,[[],["result",4]]],[11,"type_id","","",25,[[],["typeid",3]]],[11,"from","","",26,[[]]],[11,"into","","",26,[[]]],[11,"to_owned","","",26,[[]]],[11,"clone_into","","",26,[[]]],[11,"borrow","","",26,[[]]],[11,"borrow_mut","","",26,[[]]],[11,"try_from","","",26,[[],["result",4]]],[11,"try_into","","",26,[[],["result",4]]],[11,"type_id","","",26,[[],["typeid",3]]],[11,"from","","",27,[[]]],[11,"into","","",27,[[]]],[11,"to_owned","","",27,[[]]],[11,"clone_into","","",27,[[]]],[11,"borrow","","",27,[[]]],[11,"borrow_mut","","",27,[[]]],[11,"try_from","","",27,[[],["result",4]]],[11,"try_into","","",27,[[],["result",4]]],[11,"type_id","","",27,[[],["typeid",3]]],[11,"from","","",28,[[]]],[11,"into","","",28,[[]]],[11,"to_owned","","",28,[[]]],[11,"clone_into","","",28,[[]]],[11,"borrow","","",28,[[]]],[11,"borrow_mut","","",28,[[]]],[11,"try_from","","",28,[[],["result",4]]],[11,"try_into","","",28,[[],["result",4]]],[11,"type_id","","",28,[[],["typeid",3]]],[11,"from","","",29,[[]]],[11,"into","","",29,[[]]],[11,"to_owned","","",29,[[]]],[11,"clone_into","","",29,[[]]],[11,"borrow","","",29,[[]]],[11,"borrow_mut","","",29,[[]]],[11,"try_from","","",29,[[],["result",4]]],[11,"try_into","","",29,[[],["result",4]]],[11,"type_id","","",29,[[],["typeid",3]]],[11,"from","triseratops::tag::container","",31,[[]]],[11,"into","","",31,[[]]],[11,"borrow","","",31,[[]]],[11,"borrow_mut","","",31,[[]]],[11,"try_from","","",31,[[],["result",4]]],[11,"try_into","","",31,[[],["result",4]]],[11,"type_id","","",31,[[],["typeid",3]]],[11,"from","","",30,[[]]],[11,"into","","",30,[[]]],[11,"borrow","","",30,[[]]],[11,"borrow_mut","","",30,[[]]],[11,"try_from","","",30,[[],["result",4]]],[11,"try_into","","",30,[[],["result",4]]],[11,"type_id","","",30,[[],["typeid",3]]],[11,"from","triseratops::tag::analysis","",32,[[]]],[11,"into","","",32,[[]]],[11,"borrow","","",32,[[]]],[11,"borrow_mut","","",32,[[]]],[11,"try_from","","",32,[[],["result",4]]],[11,"try_into","","",32,[[],["result",4]]],[11,"type_id","","",32,[[],["typeid",3]]],[11,"from","triseratops::tag::autotags","",33,[[]]],[11,"into","","",33,[[]]],[11,"borrow","","",33,[[]]],[11,"borrow_mut","","",33,[[]]],[11,"try_from","","",33,[[],["result",4]]],[11,"try_into","","",33,[[],["result",4]]],[11,"type_id","","",33,[[],["typeid",3]]],[11,"from","triseratops::tag::beatgrid","",34,[[]]],[11,"into","","",34,[[]]],[11,"borrow","","",34,[[]]],[11,"borrow_mut","","",34,[[]]],[11,"try_from","","",34,[[],["result",4]]],[11,"try_into","","",34,[[],["result",4]]],[11,"type_id","","",34,[[],["typeid",3]]],[11,"from","","",35,[[]]],[11,"into","","",35,[[]]],[11,"borrow","","",35,[[]]],[11,"borrow_mut","","",35,[[]]],[11,"try_from","","",35,[[],["result",4]]],[11,"try_into","","",35,[[],["result",4]]],[11,"type_id","","",35,[[],["typeid",3]]],[11,"from","","",36,[[]]],[11,"into","","",36,[[]]],[11,"borrow","","",36,[[]]],[11,"borrow_mut","","",36,[[]]],[11,"try_from","","",36,[[],["result",4]]],[11,"try_into","","",36,[[],["result",4]]],[11,"type_id","","",36,[[],["typeid",3]]],[11,"from","triseratops::tag::markers","",37,[[]]],[11,"into","","",37,[[]]],[11,"borrow","","",37,[[]]],[11,"borrow_mut","","",37,[[]]],[11,"try_from","","",37,[[],["result",4]]],[11,"try_into","","",37,[[],["result",4]]],[11,"type_id","","",37,[[],["typeid",3]]],[11,"from","","",38,[[]]],[11,"into","","",38,[[]]],[11,"borrow","","",38,[[]]],[11,"borrow_mut","","",38,[[]]],[11,"try_from","","",38,[[],["result",4]]],[11,"try_into","","",38,[[],["result",4]]],[11,"type_id","","",38,[[],["typeid",3]]],[11,"from","","",39,[[]]],[11,"into","","",39,[[]]],[11,"borrow","","",39,[[]]],[11,"borrow_mut","","",39,[[]]],[11,"try_from","","",39,[[],["result",4]]],[11,"try_into","","",39,[[],["result",4]]],[11,"type_id","","",39,[[],["typeid",3]]],[11,"from","triseratops::tag::markers2","",40,[[]]],[11,"into","","",40,[[]]],[11,"borrow","","",40,[[]]],[11,"borrow_mut","","",40,[[]]],[11,"try_from","","",40,[[],["result",4]]],[11,"try_into","","",40,[[],["result",4]]],[11,"type_id","","",40,[[],["typeid",3]]],[11,"from","","",41,[[]]],[11,"into","","",41,[[]]],[11,"borrow","","",41,[[]]],[11,"borrow_mut","","",41,[[]]],[11,"try_from","","",41,[[],["result",4]]],[11,"try_into","","",41,[[],["result",4]]],[11,"type_id","","",41,[[],["typeid",3]]],[11,"from","","",42,[[]]],[11,"into","","",42,[[]]],[11,"borrow","","",42,[[]]],[11,"borrow_mut","","",42,[[]]],[11,"try_from","","",42,[[],["result",4]]],[11,"try_into","","",42,[[],["result",4]]],[11,"type_id","","",42,[[],["typeid",3]]],[11,"from","","",43,[[]]],[11,"into","","",43,[[]]],[11,"borrow","","",43,[[]]],[11,"borrow_mut","","",43,[[]]],[11,"try_from","","",43,[[],["result",4]]],[11,"try_into","","",43,[[],["result",4]]],[11,"type_id","","",43,[[],["typeid",3]]],[11,"from","","",44,[[]]],[11,"into","","",44,[[]]],[11,"borrow","","",44,[[]]],[11,"borrow_mut","","",44,[[]]],[11,"try_from","","",44,[[],["result",4]]],[11,"try_into","","",44,[[],["result",4]]],[11,"type_id","","",44,[[],["typeid",3]]],[11,"from","","",45,[[]]],[11,"into","","",45,[[]]],[11,"borrow","","",45,[[]]],[11,"borrow_mut","","",45,[[]]],[11,"try_from","","",45,[[],["result",4]]],[11,"try_into","","",45,[[],["result",4]]],[11,"type_id","","",45,[[],["typeid",3]]],[11,"from","triseratops::tag::overview","",46,[[]]],[11,"into","","",46,[[]]],[11,"borrow","","",46,[[]]],[11,"borrow_mut","","",46,[[]]],[11,"try_from","","",46,[[],["result",4]]],[11,"try_into","","",46,[[],["result",4]]],[11,"type_id","","",46,[[],["typeid",3]]],[11,"from","triseratops::tag::relvolad","",47,[[]]],[11,"into","","",47,[[]]],[11,"borrow","","",47,[[]]],[11,"borrow_mut","","",47,[[]]],[11,"try_from","","",47,[[],["result",4]]],[11,"try_into","","",47,[[],["result",4]]],[11,"type_id","","",47,[[],["typeid",3]]],[11,"from","triseratops::tag::vidassoc","",48,[[]]],[11,"into","","",48,[[]]],[11,"borrow","","",48,[[]]],[11,"borrow_mut","","",48,[[]]],[11,"try_from","","",48,[[],["result",4]]],[11,"try_into","","",48,[[],["result",4]]],[11,"type_id","","",48,[[],["typeid",3]]],[11,"parse_mp4","triseratops::tag::markers","",38,[[],[["result",4],["error",4]]]],[11,"write_mp4","","",38,[[],[["result",4],["error",4]]]],[11,"parse_ogg","triseratops::tag::analysis","",32,[[],[["result",4],["error",4]]]],[11,"write_ogg","","",32,[[],[["result",4],["error",4]]]],[11,"parse_ogg","triseratops::tag::markers2","",43,[[],[["result",4],["error",4]]]],[11,"write_ogg","","",43,[[],[["result",4],["error",4]]]],[11,"parse","triseratops::tag::analysis","",32,[[],[["result",4],["error",4]]]],[11,"write","","",32,[[],[["result",4],["error",4]]]],[11,"parse","triseratops::tag::autotags","",33,[[],[["result",4],["error",4]]]],[11,"write","","",33,[[],[["result",4],["error",4]]]],[11,"parse","triseratops::tag::beatgrid","",36,[[],[["result",4],["error",4]]]],[11,"write","","",36,[[],[["result",4],["error",4]]]],[11,"parse","triseratops::tag::markers","",38,[[],[["result",4],["error",4]]]],[11,"write","","",38,[[],[["result",4],["error",4]]]],[11,"parse","triseratops::tag::markers2","",43,[[],[["result",4],["error",4]]]],[11,"write","","",43,[[],[["result",4],["error",4]]]],[11,"parse","triseratops::tag::overview","",46,[[],[["result",4],["error",4]]]],[11,"write","","",46,[[],[["result",4],["error",4]]]],[11,"parse","triseratops::tag::relvolad","",47,[[],[["result",4],["error",4]]]],[11,"write","","",47,[[],[["result",4],["error",4]]]],[11,"parse","triseratops::tag::vidassoc","",48,[[],[["result",4],["error",4]]]],[11,"write","","",48,[[],[["result",4],["error",4]]]],[11,"from","triseratops::error","",0,[[["error",3]]]],[11,"from","","",0,[[["err",4],["verboseerror",3]]]],[11,"clone","triseratops::tag::generic","",21,[[],["color",3]]],[11,"clone","","",23,[[],["cue",3]]],[11,"clone","","",24,[[],["loop",3]]],[11,"clone","","",25,[[],["flip",3]]],[11,"clone","","",29,[[],["flipaction",4]]],[11,"clone","","",26,[[],["censorflipaction",3]]],[11,"clone","","",27,[[],["jumpflipaction",3]]],[11,"clone","","",28,[[],["unknownflipaction",3]]],[11,"default","triseratops::tag::container","",31,[[]]],[11,"eq","triseratops::library::filesystem","",14,[[["seratolibraryinfo",3]]]],[11,"ne","","",14,[[["seratolibraryinfo",3]]]],[11,"eq","triseratops::tag::generic","",21,[[["color",3]]]],[11,"ne","","",21,[[["color",3]]]],[11,"eq","","",22,[[["version",3]]]],[11,"ne","","",22,[[["version",3]]]],[11,"eq","triseratops::tag::markers","",39,[[["markertype",4]]]],[11,"fmt","triseratops::error","",0,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::library::database","",5,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::library::filesystem","",14,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::tag::generic","",21,[[["formatter",3]],["result",6]]],[11,"fmt","","",22,[[["formatter",3]],["result",6]]],[11,"fmt","","",23,[[["formatter",3]],["result",6]]],[11,"fmt","","",24,[[["formatter",3]],["result",6]]],[11,"fmt","","",25,[[["formatter",3]],["result",6]]],[11,"fmt","","",29,[[["formatter",3]],["result",6]]],[11,"fmt","","",26,[[["formatter",3]],["result",6]]],[11,"fmt","","",27,[[["formatter",3]],["result",6]]],[11,"fmt","","",28,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::tag::analysis","",32,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::tag::autotags","",33,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::tag::beatgrid","",34,[[["formatter",3]],["result",6]]],[11,"fmt","","",35,[[["formatter",3]],["result",6]]],[11,"fmt","","",36,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::tag::markers","",37,[[["formatter",3]],["result",6]]],[11,"fmt","","",38,[[["formatter",3]],["result",6]]],[11,"fmt","","",39,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::tag::markers2","",45,[[["formatter",3]],["result",6]]],[11,"fmt","","",40,[[["formatter",3]],["result",6]]],[11,"fmt","","",41,[[["formatter",3]],["result",6]]],[11,"fmt","","",42,[[["formatter",3]],["result",6]]],[11,"fmt","","",43,[[["formatter",3]],["result",6]]],[11,"fmt","","",44,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::tag::overview","",46,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::tag::relvolad","",47,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::tag::vidassoc","",48,[[["formatter",3]],["result",6]]],[11,"fmt","triseratops::error","",0,[[["formatter",3]],["result",6]]],[11,"source","","",0,[[],[["option",4],["error",8]]]]],"p":[[4,"Error"],[13,"VerboseParseError"],[13,"Base64DecodeError"],[13,"Base64InvalidLengthError"],[13,"EnvelopeNameMismatch"],[4,"Field"],[13,"Unknown"],[13,"UnknownBoolean"],[13,"UnknownContainerField"],[13,"UnknownContainerRField"],[13,"UnknownPathField"],[13,"UnknownU16Field"],[13,"UnknownU32Field"],[13,"UnknownTextField"],[3,"SeratoLibraryInfo"],[8,"EnvelopedTag"],[8,"FLACTag"],[8,"ID3Tag"],[8,"MP4Tag"],[8,"OggTag"],[8,"Tag"],[3,"Color"],[3,"Version"],[3,"Cue"],[3,"Loop"],[3,"Flip"],[3,"CensorFlipAction"],[3,"JumpFlipAction"],[3,"UnknownFlipAction"],[4,"FlipAction"],[4,"TagFormat"],[3,"TagContainer"],[3,"Analysis"],[3,"Autotags"],[3,"TerminalMarker"],[3,"NonTerminalMarker"],[3,"Beatgrid"],[3,"Marker"],[3,"Markers"],[4,"MarkerType"],[3,"UnknownMarker"],[3,"TrackColorMarker"],[3,"BPMLockMarker"],[3,"Markers2"],[3,"Markers2Content"],[4,"Marker"],[3,"Overview"],[3,"RelVolAd"],[3,"VidAssoc"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);