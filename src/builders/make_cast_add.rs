// export const makeCastAdd = async (
//     body: protobufs.CastAddBody,
//     dataOptions: MessageDataOptions,
//     signer: Signer,
// ): HubAsyncResult<protobufs.CastAddMessage> => {
// const data = await makeCastAddData(body, dataOptions);
// if (data.isErr()) {
// return err(data.error);
// }
// return makeMessage(data.value, signer);
// };

use ed25519_dalek::{Signer, SigningKey};
use prost::Message;
use crate::{CastAddBody, FarcasterNetwork, get_farcaster_time, HashScheme, MessageData, MessageType, SignatureScheme, SignerAddBody};
use crate::{Message as HubMessage};
use rand::rngs::OsRng;

pub struct MessageDataOptions {
    pub fid: u64,
    pub network: FarcasterNetwork,
    pub timestamp: Option<u32>,
}

pub enum MessageBodyOptions {
    CastAddBody(crate::CastAddBody),
    CastRemoveBody(crate::CastRemoveBody),
    ReactionBody(crate::ReactionBody),
    VerificationAddEthAddressBody(crate::VerificationAddEthAddressBody),
    VerificationRemoveBody(crate::VerificationRemoveBody),
    SignerAddBody(crate::SignerAddBody),
    SignerRemoveBody(crate::SignerRemoveBody),
    UserDataBody(crate::UserDataBody),
    LinkBody(crate::LinkBody),
    UsernameProofBody(crate::UserNameProof),
}


pub fn make_signer_add(body: SignerAddBody, data_options: MessageDataOptions, signer: SigningKey) -> Result<HubMessage, Box<dyn std::error::Error>> {
    let data = make_message_data(MessageBodyOptions::SignerAddBody(body), MessageType::SignerAdd, data_options).unwrap();

    let mut buf = Vec::new();
    let data_bytes = data.encode(&mut buf);

    let data_bytes = match data_bytes {
        Ok(_) => buf,
        Err(_) => {
            return Err(Box::from("lol"))
        }
    };

    let mut hasher = blake3::Hasher::new();
    hasher.update(&data_bytes);
    let mut hash_output = [0u8; 20];
    hasher.finalize_xof().fill(&mut hash_output);
    let hash_output = hash_output.to_vec();

    let signature = signer.sign(&hash_output.clone());
    let verifying_key = signer.verifying_key();
    let public_key = verifying_key.as_bytes();

    let data_test = HubMessage {
        data: Some(data),
        hash: hash_output.clone(),
        hash_scheme: HashScheme::Blake3.into(),
        signature: signature.clone().to_vec(),
        signature_scheme: SignatureScheme::Ed25519.into(),
        signer: public_key.clone().to_vec()
    };

    Ok(data_test)
}
pub fn make_cast_add(body: CastAddBody, data_options: MessageDataOptions, signer: SigningKey) -> Result<HubMessage, Box<dyn std::error::Error>> {
    // let data = make_cast_add_data(body, data_options).await;
    let ts = get_farcaster_time().unwrap();

    let data = make_cast_add_data(body, data_options).unwrap();

    let mut buf = Vec::new();

    let data_bytes = data.encode(&mut buf);

    let data_bytes = match data_bytes {
        Ok(_) => buf,
        Err(e) => {
            return Err(Box::from(String::from("lol")))
        }
    };

    // We need to create a hasher here with a set size of 20, since in TS we set dkLen = 20
    let mut hasher = blake3::Hasher::new();
    hasher.update(&data_bytes);
    let mut hash_output = [0u8; 20];
    hasher.finalize_xof().fill(&mut hash_output);
    let hash_output = hash_output.to_vec();

    // let signature = signer.sign_message_hash(hash_output.clone()).unwrap();
    let signature = signer.sign(&hash_output.clone());
    let verifying_key = signer.verifying_key();
    let public_key = verifying_key.as_bytes();

    let data_test = HubMessage {
        data: Some(data),
        hash: hash_output.clone(),
        hash_scheme: HashScheme::Blake3.into(),
        signature: signature.clone().to_vec(),
        signature_scheme: SignatureScheme::Ed25519.into(),
        signer: public_key.clone().to_vec(),
    };

    Ok(data_test)
}

pub fn make_cast_add_data(body: CastAddBody, data_options: MessageDataOptions) -> Result<MessageData, Box<dyn std::error::Error>> {
    let message = make_message_data(MessageBodyOptions::CastAddBody(body), MessageType::CastAdd, data_options);
    message
}

pub fn make_message_data(body_options: MessageBodyOptions, message_type: MessageType, data_options: MessageDataOptions) -> Result<MessageData, Box<dyn std::error::Error>> {
    let timestamp = match data_options.timestamp {
        Some(ts) => ts,
        None => {
            match get_farcaster_time() {
                Some(fct) => fct as u32,
                None => {
                    return Err(Box::from(String::from("sad")))
                }
            }
        }
    };

    let body = match body_options {
        MessageBodyOptions::CastAddBody(inner) => Some(crate::message_data::Body::CastAddBody(inner)),
        MessageBodyOptions::CastRemoveBody(inner) => Some(crate::message_data::Body::CastRemoveBody(inner)),
        MessageBodyOptions::ReactionBody(inner) => Some(crate::message_data::Body::ReactionBody(inner)),
        MessageBodyOptions::VerificationAddEthAddressBody(inner) => Some(crate::message_data::Body::VerificationAddEthAddressBody(inner)),
        MessageBodyOptions::VerificationRemoveBody(inner) => Some(crate::message_data::Body::VerificationRemoveBody(inner)),
        MessageBodyOptions::SignerAddBody(inner) => Some(crate::message_data::Body::SignerAddBody(inner)),
        MessageBodyOptions::SignerRemoveBody(inner) => Some(crate::message_data::Body::SignerRemoveBody(inner)),
        MessageBodyOptions::UserDataBody(inner) => Some(crate::message_data::Body::UserDataBody(inner)),
        MessageBodyOptions::LinkBody(inner) => Some(crate::message_data::Body::LinkBody(inner)),
        MessageBodyOptions::UsernameProofBody(inner) => Some(crate::message_data::Body::UsernameProofBody(inner)),
    };

    let data = MessageData {
        r#type: message_type as i32,
        fid: data_options.fid,
        timestamp,
        network: data_options.network.into(),
        body,
    };

    Ok(data)
}

pub fn validate_message_data(data: MessageData) -> Result<MessageData, Box<dyn std::error::Error>> {
    // Some key differences here between this implementation and the hub-nodejs implementation:
    // 1. We don't need to validate the FID. The key checks there are: Is number, is greater than 0, is integer. A u32 type covers all of that w/o needing additional logic


    let farcaster_time = match get_farcaster_time() {
        Some(fcts) => fcts as u32,
        None => return Err(Box::from(String::from("No Farcaster Time?")))
    };

    if data.timestamp - farcaster_time > 10 * 60 {
        return Err(Box::from(String::from("timestamp more than 10 mins in the future")))
    }

    // Validate Network
    if FarcasterNetwork::from_i32(data.network).is_none() {
        return Err(Box::from(String::from("invalid network")));
    }

    // Validate Message Type
    if MessageType::from_i32(data.r#type).is_none() {
        return Err(Box::from(String::from("invalid message type")))
    }


    Ok(data)
}

// const makeMessageData = async <TData extends protobufs.MessageData>(
// bodyOptions: MessageBodyOptions,
// messageType: protobufs.MessageType,
// dataOptions: MessageDataOptions,
// ): HubAsyncResult<TData> => {
// if (!dataOptions.timestamp) {
// getFarcasterTime().map((timestamp) => {
// dataOptions.timestamp = timestamp;
// });
// }
//
// const data = protobufs.MessageData.create({
// ...bodyOptions,
// type: messageType,
// ...dataOptions,
// });
//
// return validations.validateMessageData(data as TData);
// };
