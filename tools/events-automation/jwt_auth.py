import jwt
import os
import datetime

#from dotenv import load_dotenv
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.backends import default_backend

# Automate loading environment variables in Python script, make them accessible to the project
#load_dotenv()

def get_PEM_private_key():
    """
    Loads the PRIVATE_KEY in string from .env file.
    Returns it in PEM-formatted bytes
    """
    pem_bytes = (os.getenv('PRIVATE_KEY', "")).encode() 
    return pem_bytes

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
    AUTHORIZED_MEMBER_ID = os.getenv('AUTHORIZED_MEMBER_ID', "") # the member id that owns the OAuth Client
    CLIENT_KEY = os.getenv('CLIENT_KEY', "")
    private_key = get_RSA_private_key()
    payload = {
        "sub": AUTHORIZED_MEMBER_ID,
        "iss": CLIENT_KEY,
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
