/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This software may be used and distributed according to the terms of the
 * GNU General Public License version 2.
 */

use bytes::Bytes;
#[cfg(any(test, feature = "for-tests"))]
use quickcheck::Arbitrary;
use serde_derive::{Deserialize, Serialize};

use crate::{
    file::{
        FileAttributes, FileContent, FileEntry, FileRequest, FileSpec, HgFilenodeData,
        UploadHgFilenodeRequest, UploadHgFilenodeResponse,
    },
    wire::{
        is_default, ToApi, ToWire, WireHgId, WireKey, WireParents, WireRevisionstoreMetadata,
        WireToApiConversionError, WireUploadToken,
    },
};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct WireFileEntry {
    #[serde(rename = "0", default, skip_serializing_if = "is_default")]
    key: WireKey,

    #[serde(rename = "1", default, skip_serializing_if = "is_default")]
    data: Option<Bytes>,

    #[serde(rename = "2", default, skip_serializing_if = "is_default")]
    parents: WireParents,

    #[serde(rename = "3", default, skip_serializing_if = "is_default")]
    metadata: Option<WireRevisionstoreMetadata>,
}

impl ToWire for FileEntry {
    type Wire = WireFileEntry;

    fn to_wire(self) -> Self::Wire {
        let (data, metadata) = self
            .content
            .map_or((None, None), |c| (Some(c.hg_file_blob), Some(c.metadata)));
        WireFileEntry {
            key: self.key.to_wire(),
            parents: self.parents.to_wire(),
            data,
            metadata: metadata.to_wire(),
        }
    }
}

impl ToApi for WireFileEntry {
    type Api = FileEntry;
    type Error = WireToApiConversionError;

    fn to_api(self) -> Result<Self::Api, Self::Error> {
        let content = if let Some(hg_file_blob) = self.data {
            Some(FileContent {
                hg_file_blob,
                metadata: self
                    .metadata
                    .ok_or(WireToApiConversionError::CannotPopulateRequiredField(
                        "content.metadata",
                    ))?
                    .to_api()?,
            })
        } else {
            None
        };
        Ok(FileEntry {
            key: self.key.to_api()?,
            // if content is present, metadata must be also
            content,
            parents: self.parents.to_api()?,
        })
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct WireFileAttributes {
    #[serde(rename = "0", default, skip_serializing_if = "is_default")]
    pub content: bool,
}

impl ToWire for FileAttributes {
    type Wire = WireFileAttributes;

    fn to_wire(self) -> Self::Wire {
        WireFileAttributes {
            content: self.content,
        }
    }
}

impl ToApi for WireFileAttributes {
    type Api = FileAttributes;
    type Error = WireToApiConversionError;

    fn to_api(self) -> Result<Self::Api, Self::Error> {
        Ok(FileAttributes {
            content: self.content,
        })
    }
}

#[cfg(any(test, feature = "for-tests"))]
impl Arbitrary for WireFileAttributes {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        Self {
            content: Arbitrary::arbitrary(g),
        }
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct WireFileSpec {
    #[serde(rename = "0", default, skip_serializing_if = "is_default")]
    pub key: WireKey,

    #[serde(rename = "1", default, skip_serializing_if = "is_default")]
    pub attrs: WireFileAttributes,
}

impl ToWire for FileSpec {
    type Wire = WireFileSpec;

    fn to_wire(self) -> Self::Wire {
        WireFileSpec {
            key: self.key.to_wire(),
            attrs: self.attrs.to_wire(),
        }
    }
}

impl ToApi for WireFileSpec {
    type Api = FileSpec;
    type Error = WireToApiConversionError;

    fn to_api(self) -> Result<Self::Api, Self::Error> {
        Ok(FileSpec {
            key: self.key.to_api()?,
            attrs: self.attrs.to_api()?,
        })
    }
}

#[cfg(any(test, feature = "for-tests"))]
impl Arbitrary for WireFileSpec {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        Self {
            key: Arbitrary::arbitrary(g),
            attrs: Arbitrary::arbitrary(g),
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct WireFileRequest {
    #[serde(rename = "0", default, skip_serializing_if = "is_default")]
    pub keys: Vec<WireKey>,

    #[serde(rename = "1", default, skip_serializing_if = "is_default")]
    pub reqs: Vec<WireFileSpec>,
}

impl ToWire for FileRequest {
    type Wire = WireFileRequest;

    fn to_wire(self) -> Self::Wire {
        WireFileRequest {
            keys: self.keys.to_wire(),
            reqs: self.reqs.to_wire(),
        }
    }
}

impl ToApi for WireFileRequest {
    type Api = FileRequest;
    type Error = WireToApiConversionError;

    fn to_api(self) -> Result<Self::Api, Self::Error> {
        Ok(FileRequest {
            keys: self.keys.to_api()?,
            reqs: self.reqs.to_api()?,
        })
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct WireHgFilenodeData {
    #[serde(rename = "0", default, skip_serializing_if = "is_default")]
    pub node_id: WireHgId,

    #[serde(rename = "1", default, skip_serializing_if = "is_default")]
    pub parents: WireParents,

    #[serde(rename = "2", default, skip_serializing_if = "is_default")]
    pub file_content_upload_token: WireUploadToken,

    #[serde(rename = "3", default, skip_serializing_if = "is_default")]
    pub metadata: Vec<u8>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct WireUploadHgFilenodeRequest {
    #[serde(rename = "0", default, skip_serializing_if = "is_default")]
    pub data: WireHgFilenodeData,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct WireUploadHgFilenodeResponse {
    #[serde(rename = "1")]
    pub index: usize,

    #[serde(rename = "2", default, skip_serializing_if = "is_default")]
    pub token: WireUploadToken,
}

impl ToWire for HgFilenodeData {
    type Wire = WireHgFilenodeData;

    fn to_wire(self) -> Self::Wire {
        WireHgFilenodeData {
            node_id: self.node_id.to_wire(),
            parents: self.parents.to_wire(),
            file_content_upload_token: self.file_content_upload_token.to_wire(),
            metadata: self.metadata.to_wire(),
        }
    }
}

impl ToApi for WireHgFilenodeData {
    type Api = HgFilenodeData;
    type Error = WireToApiConversionError;

    fn to_api(self) -> Result<Self::Api, Self::Error> {
        Ok(HgFilenodeData {
            node_id: self.node_id.to_api()?,
            parents: self.parents.to_api()?,
            file_content_upload_token: self.file_content_upload_token.to_api()?,
            metadata: self.metadata.to_api()?,
        })
    }
}

impl ToWire for UploadHgFilenodeResponse {
    type Wire = WireUploadHgFilenodeResponse;

    fn to_wire(self) -> Self::Wire {
        WireUploadHgFilenodeResponse {
            index: self.index,
            token: self.token.to_wire(),
        }
    }
}

impl ToApi for WireUploadHgFilenodeResponse {
    type Api = UploadHgFilenodeResponse;
    type Error = WireToApiConversionError;

    fn to_api(self) -> Result<Self::Api, Self::Error> {
        Ok(UploadHgFilenodeResponse {
            index: self.index,
            token: self.token.to_api()?,
        })
    }
}

impl ToWire for UploadHgFilenodeRequest {
    type Wire = WireUploadHgFilenodeRequest;

    fn to_wire(self) -> Self::Wire {
        WireUploadHgFilenodeRequest {
            data: self.data.to_wire(),
        }
    }
}

impl ToApi for WireUploadHgFilenodeRequest {
    type Api = UploadHgFilenodeRequest;
    type Error = WireToApiConversionError;

    fn to_api(self) -> Result<Self::Api, Self::Error> {
        Ok(UploadHgFilenodeRequest {
            data: self.data.to_api()?,
        })
    }
}

#[cfg(any(test, feature = "for-tests"))]
impl Arbitrary for WireFileEntry {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let bytes: Option<Vec<u8>> = Arbitrary::arbitrary(g);
        Self {
            key: Arbitrary::arbitrary(g),
            data: bytes.map(Bytes::from),
            parents: Arbitrary::arbitrary(g),
            metadata: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(any(test, feature = "for-tests"))]
impl Arbitrary for WireFileRequest {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        Self {
            keys: Arbitrary::arbitrary(g),
            reqs: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(any(test, feature = "for-tests"))]
impl Arbitrary for WireUploadHgFilenodeRequest {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        Self {
            data: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(any(test, feature = "for-tests"))]
impl Arbitrary for WireHgFilenodeData {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        Self {
            node_id: Arbitrary::arbitrary(g),
            parents: Arbitrary::arbitrary(g),
            file_content_upload_token: Arbitrary::arbitrary(g),
            metadata: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::wire::tests::{check_serialize_roundtrip, check_wire_roundtrip};

    use quickcheck::quickcheck;

    quickcheck! {
        fn test_request_roundtrip_serialize(v: WireFileRequest) -> bool {
            check_serialize_roundtrip(v)
        }

        fn test_request_roundtrip_wire(v: FileRequest) -> bool {
            check_wire_roundtrip(v)
        }

        fn test_entry_roundtrip_serialize(v: WireFileEntry) -> bool {
            check_serialize_roundtrip(v)
        }

        fn test_entry_roundtrip_wire(v: FileEntry) -> bool {
            check_wire_roundtrip(v)
        }

        fn test_upload_hg_filenode_request_roundtrip_serialize(v: WireUploadHgFilenodeRequest) -> bool {
            check_serialize_roundtrip(v)
        }

        fn test_upload_hg_filenode_request_roundtrip_wire(v: UploadHgFilenodeRequest) -> bool {
            check_wire_roundtrip(v)
        }
    }
}
