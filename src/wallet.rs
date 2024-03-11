use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Digest, Sha3_256};
use hex::{encode, decode};

pub struct Wallet {
    private_key: String,
    public_key: String,
    public_address: String,
}

impl Wallet {
    pub fn new(private_key: Option<String>) -> Self {
        let secp = Secp256k1::new();
        let (private_key, public_key) = match private_key {
            Some(key_str) => {
                // Decode the provided private key string and create a SecretKey
                let private_key = SecretKey::from_slice(&decode(key_str).expect("Invalid private key format")).expect("Invalid private key");
                let public_key = PublicKey::from_secret_key(&secp, &private_key);
                (private_key, public_key)
            },
            None => {
                // Generate a new keypair
                let (private_key, public_key) = secp.generate_keypair(&mut OsRng::default());
                (private_key, public_key)
            },
        };
        let public_key_hex = encode(public_key.serialize_uncompressed()[1..].to_vec());

        let public_address = Wallet::pub_key_to_address(&public_key_hex);

        Wallet {
            private_key: private_key.display_secret().to_string(),
            public_key: public_key_hex,
            public_address,
        }
    }

    fn pub_key_to_address(public_key: &str) -> String {
        let digest = Sha3_256::digest(&decode(public_key).expect("Invalid hex in public key"));
        let hex_digest = encode(&digest);
        if hex_digest.len() >= 40 {
            // Ensure the string is long enough before slicing to avoid panics
            format!("hx{}", &hex_digest[hex_digest.len() - 40..])
        } else {
            panic!("Digest too short");
        }
    }

    // Accessor methods
    pub fn get_private_key(&self) -> String {
        self.private_key.clone()
    }

    pub fn get_public_key(&self) -> String {
        self.public_key.clone()
    }

    pub fn get_public_address(&self) -> String {
        self.public_address.clone()
    }
}
