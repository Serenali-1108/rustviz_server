use sha2::{Sha512, Digest};
use uuid::Uuid;

// REQUIRES: Non-empty, UTF-8 password
// MODIFIES: n/a
// EFFECTS: Encrypt password using SHA512
//          Return password string with $-delimited fields
//              i.e.: sha512$<unique salt>$<hashed password>
pub fn format_credentials(password: &str) -> String {
    // Hash password with unique salt
    let salt = Uuid::new_v4().to_simple().encode_lower(&mut Uuid::encode_buffer()).to_string();
    let mut hasher = Sha512::new();
    let salted_pass = salt.clone() + password;
    hasher.update(salted_pass.as_bytes());
    let hashed_final = format!("{:x}", hasher.finalize());

    [   "sha512",
        &salt,
        &hashed_final
    ].join("$")
}


// REQUIRES: Non-empty, UTF-8 passwords
// MODIFIES: n/a
// EFFECTS: Compare input password with DB password
//          Decrypts using unique salt and returns true
//          if passwords are the same
pub fn correct_cred(pass_in: &String, password_db: String) -> bool {

    // get original salt
    let v: Vec<&str> = password_db.split("$").collect();
    let salt = v[1].to_string();

    // encrypt input
    let mut hasher = Sha512::new();
    let salted_pass = salt + pass_in;
    hasher.update(salted_pass.as_bytes());
    let hashed_in = format!("{:x}", hasher.finalize());

    // v2 := original password
    hashed_in == v[2]
}
