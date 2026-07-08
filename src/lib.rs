use std::io;
use std::path::Path;

use io::BufWriter;
use io::Write;

use std::os::unix::net::UnixDatagram;

pub fn bytes2str2wtr<W>(bytes: &[u8], wtr: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    let s: &str = std::str::from_utf8(bytes).unwrap_or_default();
    let nolf: &str = s.trim_end();
    writeln!(wtr, "{nolf}")?;
    Ok(())
}

pub fn dgram2bytes(dgram: &UnixDatagram, buf: &mut [u8; 65536]) -> Result<usize, io::Error> {
    let ms: &mut [u8] = buf.as_mut_slice();
    dgram.recv(ms)
}

pub fn dgram2bytes2lines2wtr_forever<W>(
    dgram: &UnixDatagram,
    buf: &mut [u8; 65536],
    wtr: &mut W,
) -> Result<(), io::Error>
where
    W: Write,
{
    loop {
        let sz: usize = dgram2bytes(dgram, buf)?;
        let got: &[u8] = &buf[..sz];
        bytes2str2wtr(got, wtr)?;
        wtr.flush()?;
    }
}

pub fn path2dgram2bytes2lines2wtr_forever<P, W>(
    path2dgram: P,
    buf: &mut [u8; 65536],
    wtr: &mut W,
) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    W: Write,
{
    let ud: UnixDatagram = UnixDatagram::bind(path2dgram)?;
    dgram2bytes2lines2wtr_forever(&ud, buf, wtr)?;
    wtr.flush()
}

pub fn path2dgram2bytes2lines2stdout_forever_default<P>(path2dgram: P) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    let mut buf: [u8; 65536] = [0; 65536];
    let o = io::stdout();
    let mut ol = o.lock();
    let mut bw = BufWriter::new(&mut ol);
    path2dgram2bytes2lines2wtr_forever(path2dgram, &mut buf, &mut bw)?;
    drop(bw);
    ol.flush()
}
