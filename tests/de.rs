extern crate pandoc_ast;
extern crate serde_json;

use pandoc_ast::*;

#[test]
fn format() {
    let s = r#""hello""#;
    let format: Format = serde_json::from_str(s).unwrap();
    assert_eq!(format.0, "hello");
}

/*
#[test]
fn citation() {
    let s = r###"[{"unMeta":{}},[{"Para":[{"Cite":[[{"citationHash":1,"citationId":"scala_plugin","citationMode":{"NormalCitation":[]},"citationNoteNum":0,"citationPrefix":[],"citationSuffix":[]}],[{"Link":[["",[],[]],[{"Str":"1"}],["#ref-scala_plugin",""]]}]]}]}]]"###;
    filter(s.to_string(), |x| x);
}

#[test]
fn full_citation() {
    let s = r###"[{"unMeta":{"bibliography":{"MetaString":"bibliography.bib"},"csl":{"MetaString":"springer-basic-brackets-no-et-al-alphabetical.csl"},"link-citations":{"MetaBool":true}}},[{"Para":[{"Cite":[[{"citationHash":1,"citationId":"scala_plugin","citationMode":{"NormalCitation":[]},"citationNoteNum":0,"citationPrefix":[],"citationSuffix":[]}],[{"Str":"["},{"Link":[["",[],[]],[{"Str":"1"}],["#ref-scala_plugin",""]]},{"Str":"]"}]]}]},{"Header":[1,["literatur",["unnumbered"],[]],[{"Str":"Literatur"}]]},{"Para":[{"Str":" "},{"LineBreak":[]}]},{"Div":[["refs",["references"],[]],[{"Div":[["ref-scala_plugin",[],[]],[{"Para":[{"Str":"1"},{"Link":[["",[],[]],[{"Str":"text"}],["addr",""]]},{"Str":"."},{"Space":[]}]}]]}]]}]]"###;
    filter(s.to_string(), |x| x);
}*/
/*
#[test]
fn image() {
    //let s = r####"[{"unMeta":{"date":{"t":"MetaInlines","c":[{"t":"Str","c":"Dr.-Ing."},{"t":"Space","c":[]},{"t":"Str","c":"Jörg"},{"t":"Space","c":[]},{"t":"Str","c":"Matthes"},{"t":"LineBreak","c":[]},{"t":"Str","c":"Dipl.-Inf."},{"t":"Space","c":[]},{"t":"Str","c":"Oliver"},{"t":"Space","c":[]},{"t":"Str","c":"Schneider"}]},"author":{"t":"MetaList","c":[{"t":"MetaInlines","c":[{"t":"Str","c":"Einführung"},{"t":"Space","c":[]},{"t":"Str","c":"C"}]}]},"title":{"t":"MetaInlines","c":[{"t":"Str","c":"Grundlagen"},{"t":"Space","c":[]},{"t":"Str","c":"der"},{"t":"Space","c":[]},{"t":"Str","c":"Informatik"},{"t":"LineBreak","c":[]},{"t":"Str","c":"Teil"},{"t":"Space","c":[]},{"t":"Str","c":"1"}]}}},[{"t":"Header","c":[1,["grundlagen",[],[]],[{"t":"Str","c":"Grundlagen"}]]},{"t":"Header","c":[2,["einführung-programmiersprachen",[],[]],[{"t":"Str","c":"Einführung"},{"t":"Space","c":[]},{"t":"Str","c":"Programmiersprachen"}]]},{"t":"Para","c":[{"t":"Image","c":[[],["ProgrammiersprachenVerwandschaft.png","fig:"]]}]},{"t":"Header","c":[1,["baaa",[],[]],[{"t":"Str","c":"BAAA"}]]},{"t":"BulletList","c":[[{"t":"Plain","c":[{"t":"Str","c":"bee"}]}],[{"t":"Plain","c":[{"t":"Str","c":"boo"}]}]]},{"t":"Header","c":[1,["bool",[],[]],[{"t":"Str","c":"Bool"}]]},{"t":"Para","c":[{"t":"Str","c":"bar"}]}]]"####;
    let s = r####"{"t":"Image","c":[[],["ProgrammiersprachenVerwandschaft.png","fig:"]]}"####;
    let mut value: serde_json::Value = serde_json::from_str(s).unwrap();
    println!("{:?}", value);
    pandoc_to_serde(&mut value);
    println!("{:?}", value);
    let _: Inline = serde_json::from_value(value).unwrap();
}

*/

#[test]
fn one_point_twenty_two_tables() {
    let s = r####"{"pandoc-api-version":[1,22],"meta":{},"blocks":[{"t":"Table","c":[["",[],[]],[null,[{"t":"Para","c":[{"t":"Str","c":"Data about the planets of our solar system."}]}]],[[{"t":"AlignCenter"},{"t":"ColWidthDefault"}],[{"t":"AlignCenter"},{"t":"ColWidthDefault"}],[{"t":"AlignDefault"},{"t":"ColWidthDefault"}],[{"t":"AlignRight"},{"t":"ColWidthDefault"}],[{"t":"AlignRight"},{"t":"ColWidthDefault"}],[{"t":"AlignRight"},{"t":"ColWidthDefault"}],[{"t":"AlignRight"},{"t":"ColWidthDefault"}],[{"t":"AlignRight"},{"t":"ColWidthDefault"}],[{"t":"AlignRight"},{"t":"ColWidthDefault"}],[{"t":"AlignRight"},{"t":"ColWidthDefault"}],[{"t":"AlignRight"},{"t":"ColWidthDefault"}],[{"t":"AlignDefault"},{"t":"ColWidthDefault"}]],[["",[],[]],[[["",[],[]],[[["",[],[]],{"t":"AlignDefault"},1,2,[{"t":"Plain","c":[{"t":"Str","c":""}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Name"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Mass (10^24kg)"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Diameter (km)"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Density (kg/m^3)"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Gravity (m/s^2)"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Length of day (hours)"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Distance from Sun (10^6km)"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Mean temperature (C)"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Number of moons"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Notes"}]}]]]]]],[[["",[],[]],3,[],[[["",[],[]],[[["",[],[]],{"t":"AlignDefault"},4,2,[{"t":"Plain","c":[{"t":"Str","c":"Terrestrial planets"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Mercury"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"0.330"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"4,879"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"5427"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"3.7"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"4222.6"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"57.9"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"167"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"0"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Closest to the Sun"}]}]]]],[["",[],[]],[[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Venus"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"4.87"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"12,104"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"5243"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"8.9"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"2802.0"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"108.2"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"464"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"0"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":""}]}]]]],[["",[],[]],[[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Earth"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"5.97"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"12,756"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"5514"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"9.8"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"24.0"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"149.6"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"15"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"1"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Our world"}]}]]]],[["",[],[]],[[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Mars"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"0.642"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"6,792"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"3933"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"3.7"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"24.7"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"227.9"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"-65"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"2"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"The red planet"}]}]]]],[["",[],[]],[[["",[],[]],{"t":"AlignDefault"},4,1,[{"t":"Plain","c":[{"t":"Str","c":"Jovian planets"}]}]],[["",[],[]],{"t":"AlignDefault"},2,1,[{"t":"Plain","c":[{"t":"Str","c":"Gas giants"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Jupiter"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"1898"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"142,984"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"1326"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"23.1"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"9.9"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"778.6"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"-110"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"67"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"The largest planet"}]}]]]],[["",[],[]],[[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Saturn"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"568"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"120,536"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"687"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"9.0"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"10.7"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"1433.5"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"-140"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"62"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":""}]}]]]],[["",[],[]],[[["",[],[]],{"t":"AlignDefault"},2,1,[{"t":"Plain","c":[{"t":"Str","c":"Ice giants"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Uranus"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"86.8"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"51,118"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"1271"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"8.7"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"17.2"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"2872.5"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"-195"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"27"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":""}]}]]]],[["",[],[]],[[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Neptune"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"102"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"49,528"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"1638"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"11.0"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"16.1"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"4495.1"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"-200"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"14"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":""}]}]]]],[["",[],[]],[[["",[],[]],{"t":"AlignDefault"},1,2,[{"t":"Plain","c":[{"t":"Str","c":"Dwarf planets"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Pluto"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"0.0146"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"2,370"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"2095"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"0.7"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"153.3"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"5906.4"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"-225"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"5"}]}]],[["",[],[]],{"t":"AlignDefault"},1,1,[{"t":"Plain","c":[{"t":"Str","c":"Declassified as a planet in 2006."}]}]]]]]]],[["",[],[]],[]]]}]}"####;
    let value: serde_json::Value = serde_json::from_str(s).unwrap();
    println!("{:?}", value);
    let _: Pandoc = serde_json::from_value(value).unwrap();
}
