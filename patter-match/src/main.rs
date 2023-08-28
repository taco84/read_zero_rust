#[derive(Debug)]
enum Storage {
    _HDD{ size: u32, rpm: u32},
    SSD(u32),
}

#[derive(Debug)]
struct PCSpec {
    cpu : u16,
    memory : u32,
    storage : Storage,
}

fn main() {
    let spec = PCSpec{
        cpu: 8,
        memory: 256,
        storage: Storage::SSD(512),
    };

    match spec {
        PCSpec {
            storage: Storage::SSD(512),
            ..
        } => {
            println!("512GiB SSD");
        }
        PCSpec {
            cpu: 4 | 8,
            memory: m,
            storage: _,
        } => {
            println!("4 or 8 CPU");
            println!("{}GiB memory",m);
        }
        PCSpec { memory: m, .. } if m < 4 => {
            println!("4GiBより少ないメモリ");
        }
        _ => ()
    }

    println!("{:?}",spec);
}