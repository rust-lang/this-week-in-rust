import jwt
import os
import datetime

from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.backends import default_backend

MEETUP_PEM_ENV_VAR = "MEETUP_PRIVATE_KEY"
MEETUP_AUTHORIZED_MEMBER_ID_ENV_VAR = "MEETUP_AUTHORIZED_MEMBER_ID"
MEETUP_CLIENT_KEY_ENV_VAR = "MEETUP_CLIENT_KEY"


def get_PEM_private_key():
    """
    Loads the PRIVATE_KEY in string from .env file.
    Returns it in PEM-formatted bytes
    """
    pem_env_var = os.getenv(MEETUP_PEM_ENV_VAR)
    if not pem_env_var:
        raise RuntimeError(f"Env var {MEETUP_PEM_ENV_VAR} not set!")

    with open(pem_env_var, "rb") as pem_file:
        return pem_file.read()

def get_RSA_private_key():
    """
    Deserializes and sign the private key in PEM-formatted in bytes to an RSA private key object using cryptographic operations.
    Returns the RSA private key object
    """
    private_key = serialization.load_pem_private_key(
        get_PEM_private_key(), password=None, backend=default_backend()
    )
    return private_key

def get_RSA_public_key():
    """
    Returns the corresponding RSA public key object from RSA private_key
    """
    public_key = get_RSA_private_key().public_key()
    return public_key

def get_PEM_public_key():
    """
    Returns the public key in in PEM-formatted in bytes using RSA public key, to verify digital signatures
    """
    pem_bytes = (get_RSA_public_key().public_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PublicFormat.SubjectPublicKeyInfo
    )).decode()
    return pem_bytes

# This function is essential for authorize step when calling Meetup API
def generate_signed_jwt():
    """
    Generates a JWT: 
    Encodes and signs the payload using RS256 and the private RSA key, forming a base64-url encoded header, payload, and signature. 
    Then returns it.
    """
    authorized_member_id = os.getenv(MEETUP_AUTHORIZED_MEMBER_ID_ENV_VAR) # the member id that owns the OAuth Client
    client_key = os.getenv(MEETUP_CLIENT_KEY_ENV_VAR)

    # TODO: consolidate fetching + checking env vars into one place
    if not authorized_member_id:
        raise RuntimeError(f"Env var {MEETUP_AUTHORIZED_MEMBER_ID_ENV_VAR} not set!")

    if not client_key:
        raise RuntimeError(f"Env var {MEETUP_CLIENT_KEY_ENV_VAR} not set!")

    private_key = get_RSA_private_key()
    payload = {
        "sub": authorized_member_id,
        "iss": client_key,
        "aud": "api.meetup.com",
        "exp": (datetime.datetime.utcnow() + datetime.timedelta(hours=24)).timestamp()
    }
    return jwt.encode(
        payload=payload, 
        key=private_key, 
        algorithm="RS256"
    )

def decode_and_validate_jwt():
    """
    Checks/Validates the signed jwt.
    Returns a decoded jwt payload/claim
    """
    token = generate_signed_jwt()
    pem_public_key = get_PEM_public_key()
    try:
        payload = jwt.decode(
            token, 
            key=pem_public_key, 
            algorithms="RS256", 
            audience="api.meetup.com"
        )
        return payload
    except ExpiredSignatureError as error:
        print(f'Unable to decode the token, error: {error}')
