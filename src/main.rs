#[derive(structopt::StructOpt, Debug, Clone)]
/// コマンドライン引数
pub struct Args {
    #[structopt(long, parse(from_os_str))]
    src: std::path::PathBuf,
    #[structopt(long, parse(from_os_str))]
    dst: std::path::PathBuf,
    #[structopt(short, long)]
    r: u8,
    #[structopt(short, long)]
    g: u8,
    #[structopt(short, long)]
    b: u8,
}

fn main() {
    let arg = <Args as structopt::StructOpt>::from_args();
    let mut ply = ply::PLYFile::from_file(&arg.src).unwrap();
    for ele in ply.elements.iter_mut() {
        if let ply::Element::Element(element) = ele {
            let prop = element.property_mut();
            let mut r_index = None;
            let mut g_index = None;
            let mut b_index = None;
            for (index, (prop_name, prop_type)) in prop.iter_mut().enumerate() {
                (ply::PLYValueTypeName::Uchar == *prop_type).then(|| match prop_name.as_ref() {
                    "r" => r_index = Some(index),
                    "g" => g_index = Some(index),
                    "b" => b_index = Some(index),
                    _ => { /* do nothing */ }
                });
            }
            if r_index.is_none() {
                prop.push_prop("r", ply::PLYValueTypeName::Uchar);
            }
            if g_index.is_none() {
                prop.push_prop("g", ply::PLYValueTypeName::Uchar);
            }
            if b_index.is_none() {
                prop.push_prop("b", ply::PLYValueTypeName::Uchar);
            }

            let payloads = element.payload_mut();
            for payload in payloads.iter_mut() {
                if let Some(r_index) = r_index {
                    *payload.get_mut(r_index).unwrap() = ply::PLYValue::Uchar(arg.r);
                } else {
                    payload.push_value(ply::PLYValue::Uchar(arg.r));
                }
                if let Some(g_index) = g_index {
                    *payload.get_mut(g_index).unwrap() = ply::PLYValue::Uchar(arg.g);
                } else {
                    payload.push_value(ply::PLYValue::Uchar(arg.g));
                }
                if let Some(b_index) = b_index {
                    *payload.get_mut(b_index).unwrap() = ply::PLYValue::Uchar(arg.b);
                } else {
                    payload.push_value(ply::PLYValue::Uchar(arg.b));
                }
            }
        }
    }
    let mut writer = std::io::BufWriter::new(std::fs::File::create(&arg.dst).unwrap());
    ply.write(&mut writer).unwrap();
}
