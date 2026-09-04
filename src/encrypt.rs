use crate::{
    api_key_credential::ApiKeyCredentials,
    error::{ETError, ETResult},
};
use hmac::{Hmac, Mac};
use p256::ecdsa::signature::Signer as SignerTrait;
use rsa::{
    RsaPrivateKey, pkcs1::DecodeRsaPrivateKey, pkcs1v15::SigningKey as RsaPkcs1v15SigningKey,
    pkcs8::DecodePrivateKey,
};
use secrecy::{ExposeSecret, SecretSlice};
use sha2::{Sha256, Sha512};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptionAlgorithm {
    EcdsaP256,
    EcdsaP384,
    Ed25519,
    HmacSha256,
    HmacSha512,
    RsaSha256,
}

#[derive(Display)]
pub enum Encryptor {
    EcdsaP256(p256::ecdsa::SigningKey),
    EcdsaP384(p384::ecdsa::SigningKey),
    Ed25519(ed25519_compact::SecretKey),
    HmacSha256(secrecy::SecretSlice<u8>),
    HmacSha512(secrecy::SecretSlice<u8>),
    RsaSha256(Box<RsaPkcs1v15SigningKey<Sha256>>),
}

impl std::fmt::Debug for Encryptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Encryptor::EcdsaP256(_) => write!(f, "EcdsaP256"),
            Encryptor::EcdsaP384(_) => write!(f, "EcdsaP384"),
            Encryptor::Ed25519(_) => write!(f, "Ed25519"),
            Encryptor::HmacSha256(_) => write!(f, "HmacSha256"),
            Encryptor::HmacSha512(_) => write!(f, "HmacSha512"),
            Encryptor::RsaSha256(_) => write!(f, "RsaSha256"),
        }
    }
}

impl EncryptionAlgorithm {
    pub fn encryptor(&self, api_key_credentials: ApiKeyCredentials) -> ETResult<Encryptor> {
        let secret_key_bytes = api_key_credentials.secret.expose_secret().as_bytes();
        match self {
            Self::EcdsaP256 => {
                let signing_key = p256::ecdsa::SigningKey::from_slice(secret_key_bytes)
                    .map_err(|e| ETError::CryptoKey(format!("ECDSA P-256 key error: {e}")))?;
                Ok(Encryptor::EcdsaP256(signing_key))
            }
            Self::EcdsaP384 => {
                let signing_key = p384::ecdsa::SigningKey::from_slice(secret_key_bytes)
                    .map_err(|e| ETError::CryptoKey(format!("ECDSA P-384 key error: {e}")))?;
                Ok(Encryptor::EcdsaP384(signing_key))
            }
            Self::Ed25519 => {
                let seed = ed25519_compact::Seed::from_slice(secret_key_bytes).map_err(|_| {
                    ETError::CryptoKey("Ed25519 key must be exactly 32 bytes".to_string())
                })?;
                if seed.iter().all(|byte| *byte == 0) {
                    return Err(ETError::CryptoKey(
                        "Ed25519 key must not be all zeros".to_string(),
                    ));
                }
                let signing_key = ed25519_compact::KeyPair::from_seed(seed).sk;
                Ok(Encryptor::Ed25519(signing_key))
            }
            Self::HmacSha256 => {
                let signing_slice = SecretSlice::from(secret_key_bytes.to_vec());
                Ok(Encryptor::HmacSha256(signing_slice))
            }
            Self::HmacSha512 => {
                let signing_slice = SecretSlice::from(secret_key_bytes.to_vec());
                Ok(Encryptor::HmacSha512(signing_slice))
            }
            Self::RsaSha256 => {
                let signing_key = rsa_signing_key(api_key_credentials.secret.expose_secret())
                    .map_err(|e| ETError::CryptoKey(format!("RSA key error: {e}")))?;
                Ok(Encryptor::RsaSha256(Box::new(signing_key)))
            }
        }
    }
}

impl Encryptor {
    pub fn encrypt(&self, bytes: &[u8]) -> ETResult<Vec<u8>> {
        match self {
            Self::EcdsaP256(signing_key) => {
                let signature: p256::ecdsa::Signature = signing_key.sign(bytes);
                Ok(signature.to_der().to_bytes().to_vec())
            }
            Self::EcdsaP384(signing_key) => {
                let signature: p384::ecdsa::Signature = signing_key.sign(bytes);
                Ok(signature.to_der().to_bytes().to_vec())
            }
            Self::Ed25519(signing_key) => {
                let signature = signing_key.sign(bytes, None);
                Ok(signature.to_vec())
            }
            Self::HmacSha256(signing_slice) => {
                let mut mac = Hmac::<Sha256>::new_from_slice(signing_slice.expose_secret())
                    .map_err(|e| ETError::CryptoKey(format!("HMAC-SHA256 key error: {e}")))?;
                mac.update(bytes);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            Self::HmacSha512(signing_slice) => {
                let mut mac = Hmac::<Sha512>::new_from_slice(signing_slice.expose_secret())
                    .map_err(|e| ETError::CryptoKey(format!("HMAC-SHA512 key error: {e}")))?;
                mac.update(bytes);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            Self::RsaSha256(signing_key) => {
                let signature: rsa::pkcs1v15::Signature = signing_key.sign(bytes);
                let signature: Box<[u8]> = signature.into();
                Ok(signature.to_vec())
            }
        }
    }
}

fn rsa_signing_key(secret: &str) -> Result<RsaPkcs1v15SigningKey<Sha256>, String> {
    let secret = secret.trim();
    let private_key = if secret.starts_with("-----BEGIN") {
        if secret.contains("RSA PRIVATE KEY") {
            RsaPrivateKey::from_pkcs1_pem(secret).map_err(|e| format!("invalid PKCS#1 PEM: {e}"))?
        } else {
            RsaPrivateKey::from_pkcs8_pem(secret).map_err(|e| format!("invalid PKCS#8 PEM: {e}"))?
        }
    } else {
        let bytes = secret.as_bytes();
        RsaPrivateKey::from_pkcs8_der(bytes)
            .or_else(|_| RsaPrivateKey::from_pkcs1_der(bytes))
            .map_err(|e| format!("invalid PKCS#8 or PKCS#1 DER: {e}"))?
    };
    Ok(RsaPkcs1v15SigningKey::<Sha256>::new(private_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdsa::signature::Verifier;
    use rsa::pkcs8::DecodePublicKey;
    use secrecy::SecretString;

    const RSA_PRIVATE_KEY_PKCS1_PEM: &str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAvUlSWT6Ixn1IPc0SUaH31MBcvn2dU9JYbi6R14q5K4wY5769
fGU2HfnNATXEY68N5Tm/rBSOaDUAnox17Iz6rQnqj+f3vDLl1mdUzJGY7xSM2WSy
iLGiH7xvKDqPMqgLt1aPJfK/tXf35bKNhrJJIJ84wbspxWM+TcabNuIbKoCznYDe
kbiMXHIZyTq2ohhkkbcQAZgxbF6rDZ45CJTXD4keSy+nkp2HhDjxXFHBexQWqL5f
VGc25jAZM2/OJbWLyxcVvAZegIpipSgr8BPPjfjalUiMiEQWCSIPRZCPES41DHAm
JdLuNlHwhzIOO4eXUScl96msDUQ+GGz2pZ59ewIDAQABAoIBABZXirRE+NDw5a+B
vplLFF0UzX2ghwrnR4/NyGIYGi4lLaVg8q21ppMYMpXjekhH18yIKfMORBbRtr6A
FsUyiL4W7wxIVYntugo6DRzOTK5fjxZz18zhpqC3Val6a/+PLT7ZJTFV+0HYpLaE
gkb1UUNb98+KH26QfcChYh2yx4EXFnA1RewYZdr7wJXnYCP+ypPGdjLvp3IHcgju
s2hw++h/Gw0/j3/vFfFmlsAY0lFIZDmevGocApiw7nqLkQTyqkeIvuc3krwNtlZp
GHhSiqgHGVy6j46CeFZkJDRqLNbU0g9PRpuKOExV4xWTmGl7RkoqDJFRJex7taTq
FboLTmUCgYEA+017/pIU8Sk6RqamzMEYzxmryYmXmtb42DWhJx6TLsl16dxg+M4s
myqesL+ZLvujDKrws2jU1FOMwvSeVyDkyDrfd/F2J58n+sh2AD4gLQFfUY+U+tXB
cEcbg5xbpRBNsWW3nJ0+wo0LuFveGUXJinQnmSvEu3w5Y5Ud5YAdrd8CgYEAwNMU
3yi3BIJ5nRqJhUKlHlgfmI/vKwV9yBvS5ZAtCD9bP8420mS2n6eavmnekTQlOFKo
fTZ/4OtyASDAK8uW7KBqgDCTsNyBOFF4kwBVug9yFzNXQpEZ06XHx35N15YX5qXu
mWzAlpps6L6TFZrTleiFNa4zoTPPyvReFknSq+UCgYAPv5RUqLztPAMt6EWtsTAn
0lkEoT3B81TW382AkRbyKKfeBvSAwxLiinI1a1xjKFvYcGHzLE+iGrZIhWHi3enY
bnAQqS4ZX4z211clvr7vcxWRG+lfKsQbkCk2PzczCemEg3ZVNzl2obYpUICDnu75
YTwWCjsi8K78AZ1FA5lYiwKBgF/78uTPEKOHC2Cf6BrUvPro5Pl9lJF2z1EAQLAT
X8c+On79eJ6skZfRx57HdooTSf/KbaK+sVNWAE79bYufcYZiqAEdrTcZ3J0bOQj7
3rCapbFU1jgFoLSRTmxIvBj31vfyW142G3wcbOBClVbo1jBGKaE6EOtKrkIZ+Ifh
waPdAoGBALlELsfGw99RxK0lrZSohaGALaMdiSKkmcFlaMy04SboW04M5CVQfEEl
I7pCs7Ln13XMAzkiWQelO87mCbNBmz30lYFIOhfy7DSOEdA6aVD//xSW3mkF0T1o
5qof09cC1yMJh1yVGYhwDMK9TRx1g8DBx9pAH2PpWy/GHsGG2puL
-----END RSA PRIVATE KEY-----
"#;

    const RSA_PRIVATE_KEY_PKCS8_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC9SVJZPojGfUg9
zRJRoffUwFy+fZ1T0lhuLpHXirkrjBjnvr18ZTYd+c0BNcRjrw3lOb+sFI5oNQCe
jHXsjPqtCeqP5/e8MuXWZ1TMkZjvFIzZZLKIsaIfvG8oOo8yqAu3Vo8l8r+1d/fl
so2GskkgnzjBuynFYz5Nxps24hsqgLOdgN6RuIxcchnJOraiGGSRtxABmDFsXqsN
njkIlNcPiR5LL6eSnYeEOPFcUcF7FBaovl9UZzbmMBkzb84ltYvLFxW8Bl6AimKl
KCvwE8+N+NqVSIyIRBYJIg9FkI8RLjUMcCYl0u42UfCHMg47h5dRJyX3qawNRD4Y
bPalnn17AgMBAAECggEAFleKtET40PDlr4G+mUsUXRTNfaCHCudHj83IYhgaLiUt
pWDyrbWmkxgyleN6SEfXzIgp8w5EFtG2voAWxTKIvhbvDEhVie26CjoNHM5Mrl+P
FnPXzOGmoLdVqXpr/48tPtklMVX7QdiktoSCRvVRQ1v3z4ofbpB9wKFiHbLHgRcW
cDVF7Bhl2vvAledgI/7Kk8Z2Mu+ncgdyCO6zaHD76H8bDT+Pf+8V8WaWwBjSUUhk
OZ68ahwCmLDueouRBPKqR4i+5zeSvA22VmkYeFKKqAcZXLqPjoJ4VmQkNGos1tTS
D09Gm4o4TFXjFZOYaXtGSioMkVEl7Hu1pOoVugtOZQKBgQD7TXv+khTxKTpGpqbM
wRjPGavJiZea1vjYNaEnHpMuyXXp3GD4ziybKp6wv5ku+6MMqvCzaNTUU4zC9J5X
IOTIOt938XYnnyf6yHYAPiAtAV9Rj5T61cFwRxuDnFulEE2xZbecnT7CjQu4W94Z
RcmKdCeZK8S7fDljlR3lgB2t3wKBgQDA0xTfKLcEgnmdGomFQqUeWB+Yj+8rBX3I
G9LlkC0IP1s/zjbSZLafp5q+ad6RNCU4Uqh9Nn/g63IBIMAry5bsoGqAMJOw3IE4
UXiTAFW6D3IXM1dCkRnTpcfHfk3Xlhfmpe6ZbMCWmmzovpMVmtOV6IU1rjOhM8/K
9F4WSdKr5QKBgA+/lFSovO08Ay3oRa2xMCfSWQShPcHzVNbfzYCRFvIop94G9IDD
EuKKcjVrXGMoW9hwYfMsT6IatkiFYeLd6dhucBCpLhlfjPbXVyW+vu9zFZEb6V8q
xBuQKTY/NzMJ6YSDdlU3OXahtilQgIOe7vlhPBYKOyLwrvwBnUUDmViLAoGAX/vy
5M8Qo4cLYJ/oGtS8+ujk+X2UkXbPUQBAsBNfxz46fv14nqyRl9HHnsd2ihNJ/8pt
or6xU1YATv1ti59xhmKoAR2tNxncnRs5CPvesJqlsVTWOAWgtJFObEi8GPfW9/Jb
XjYbfBxs4EKVVujWMEYpoToQ60quQhn4h+HBo90CgYEAuUQux8bD31HErSWtlKiF
oYAtox2JIqSZwWVozLThJuhbTgzkJVB8QSUjukKzsufXdcwDOSJZB6U7zuYJs0Gb
PfSVgUg6F/LsNI4R0DppUP//FJbeaQXRPWjmqh/T1wLXIwmHXJUZiHAMwr1NHHWD
wMHH2kAfY+lbL8YewYbam4s=
-----END PRIVATE KEY-----
"#;

    const RSA_PUBLIC_KEY_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvUlSWT6Ixn1IPc0SUaH3
1MBcvn2dU9JYbi6R14q5K4wY5769fGU2HfnNATXEY68N5Tm/rBSOaDUAnox17Iz6
rQnqj+f3vDLl1mdUzJGY7xSM2WSyiLGiH7xvKDqPMqgLt1aPJfK/tXf35bKNhrJJ
IJ84wbspxWM+TcabNuIbKoCznYDekbiMXHIZyTq2ohhkkbcQAZgxbF6rDZ45CJTX
D4keSy+nkp2HhDjxXFHBexQWqL5fVGc25jAZM2/OJbWLyxcVvAZegIpipSgr8BPP
jfjalUiMiEQWCSIPRZCPES41DHAmJdLuNlHwhzIOO4eXUScl96msDUQ+GGz2pZ59
ewIDAQAB
-----END PUBLIC KEY-----
"#;

    fn credentials(secret: &str) -> ApiKeyCredentials {
        ApiKeyCredentials {
            api_key: "api-key".to_string(),
            secret: SecretString::from(secret.to_string()),
        }
    }

    fn assert_rsa_signs(pem: &str) {
        let encryptor = EncryptionAlgorithm::RsaSha256
            .encryptor(credentials(pem))
            .expect("RSA private key should parse");
        let message = b"symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        let signature = encryptor.encrypt(message).expect("signing should succeed");
        assert_eq!(signature.len(), 256);
        assert_eq!(signature, encryptor.encrypt(message).unwrap());

        let public_key = rsa::RsaPublicKey::from_public_key_pem(RSA_PUBLIC_KEY_PEM)
            .expect("RSA public key should parse");
        let verifying_key = rsa::pkcs1v15::VerifyingKey::<Sha256>::new(public_key);
        let parsed_signature = rsa::pkcs1v15::Signature::try_from(signature.as_slice()).unwrap();
        verifying_key
            .verify(message, &parsed_signature)
            .expect("signature should verify");
    }

    #[test]
    fn rsa_pkcs1_pem_secret_signs() {
        assert_rsa_signs(RSA_PRIVATE_KEY_PKCS1_PEM);
    }

    #[test]
    fn rsa_pkcs8_pem_secret_signs() {
        assert_rsa_signs(RSA_PRIVATE_KEY_PKCS8_PEM);
    }

    #[test]
    fn rsa_invalid_secret_is_rejected() {
        let error = EncryptionAlgorithm::RsaSha256
            .encryptor(credentials("not-an-rsa-private-key"))
            .unwrap_err();
        assert!(matches!(error, ETError::CryptoKey(_)));
    }
}
