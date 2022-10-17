use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Telegraph Account struct
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    /// a short name on telegraph page
    pub short_name: Option<String>,
    /// author name for creating page on telegraph
    pub author_name: Option<String>,
    /// custom for user,c can be any url
    pub author_url: Option<String>,
    /// access token for telegraph
    pub access_token: Option<String>,
    /// auth to telegraph, it's can be visit in 5 min then it's expired
    pub auth_url: Option<String>,
    /// how many pages in this telegraph account
    pub page_count: Option<i32>,
}


/// PageList struct  a list of Telegraph articles belonging to an account. Most recently created articles first.
#[derive(Debug, Clone, Deserialize)]
pub struct PageList {
    /// how many pages belongs to a telegraph account
    pub total_count: i32,
    /// requested pages of the target Telegraph account.
    pub pages: Vec<Page>,
}

/// Telegraph Page struct
#[derive(Debug,Clone,Deserialize)]
pub struct Page{
    /// path to the page
    pub path:String,
    /// url for the page
    pub url:String,
    /// title of a page
    pub title:String,
    /// description of a page
    pub description:String,
    /// author name,display below page title
    pub author_name: Option<String>,
    /// author url can be any url
    pub author_url: Option<String>,
    /// image url of the page
    pub image_url: Option<String>,
    /// content of the page
    pub content:Option<Vec<Node>>,
    /// number of page views for the page
    pub view:i32,
    /// Optional. Only returned if access_token passed.
    ///
    /// true, if the target Telegraph account can edit the page.
    pub can_edit:Option<bool>,
}

/// represents the number of page views for a Telegraph article.
#[derive(Debug, Clone, Deserialize)]
pub struct PageViews{
    /// number of page views for the target page
    pub view:i32,
}

/// this is a DOM Node abstract
/// it can be a String which represents a DOM text node or a NodeElement object.
#[derive(Debug, Clone, Deserialize,Serialize)]
#[serde(untagged)]
pub enum Node{
    Text(String),
    NodeElement(NodeElement),
}

/// NodeElement struct
#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct  NodeElement{
    /// Name of the DOM element.
    /// Available tags: a, aside, b, blockquote, br, code, em, figcaption, figure, h3, h4, hr, i, iframe, img, li, ol, p, pre, s, strong, u, ul, video.
    pub tag:String,

    /// Optional. Attributes of the DOM element.
    ///
    /// Key of object represents name of attribute, value represents value of attribute.
    ///
    /// Available attributes: href, src.
    pub attrs:Option<HashMap<String,String>>,

    /// Optional. List of child nodes for the DOM element.
    pub children:Option<Vec<Node>>,
}

/// UploadResult struct
#[derive(Debug, Clone, Deserialize)]
pub enum UploadResult{
    Error{error:String},
    Source(Vec<ImageInfo>)
}

/// ImageInfo struct
pub struct ImageInfo{
    /// path of the image uploaded
    pub src:String,
}

