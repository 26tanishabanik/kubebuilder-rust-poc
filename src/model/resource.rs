use regex::Regex;

pub struct API{
    CRD_version: String,
    Namespaced: bool
}

pub struct GVK {
    Group: String,
    Domain: String,
    Version: String,
    Kind: String
}  

pub struct Resource {
    GVK: GVK,
    Plural: String,
    Path: String,
    API: *const API,
    Controller: bool,
    Webhooks: bool, // use webhook type
}

const version_pattern: &str = "^v\\d+(?:alpha\\d+|beta\\d+)?$";
const group_required: String = "group cannot be empty if the domain is empty".to_owned();
const version_required: String = "version cannot be empty".to_owned();
const kind_required: String = "kind cannot be empty".to_owned();
static mut version_regex: regex::Regex = Regex::new(version_pattern).unwrap();


impl Resource {
    fn Validate(self) -> Result<(),Box<dyn std::error::Error>>{
        if self.GVK.Validate() {
            Ok(());
        }
        if 
    }
}


impl GVK {
    fn Validate(self) -> Result<(),Box<dyn std::error::Error>> {
        if self.Qualified_group() == "" {
            Ok(());
        }
        if self.Version == "" {
            Ok(());
        }
        if !version_regex.is_match(&self.Version) {
            Ok(());
        }
        if self.Kind == "" {
            Ok(());
        }
        if &self.Kind[0..1] == &self.Kind[0..1].to_lowercase(){
            Ok(());
        }
        Ok(())
    }
    fn Qualified_group(self) -> String {
        let full_domain: String;
        let group: &str = &self.Group;
        let domain: &str = &self.Domain;
        full_domain.push_str(group);
        full_domain.push_str(".");
        full_domain.push_str(domain);
        match "" {
            self.Domain => return self.Domain,
            self.Group => return self.Group,
            _ => full_domain

        }
    }
    fn is_equal_to(self, other: GVK) -> bool {
        return self.Group == other.Group  &&
                self.Domain == other.Domain &&
                self.Version == other.Version &&
                self.Kind == other.Kind
    }
}


impl API {
    fn Validate(self) -> Result<(),Box<dyn std::error::Error>>{
        return Ok(())
    }
    fn Copy(self) -> API {
        self
    }
    fn Update(&mut self, other: *mut API) -> Result<(),Box<dyn std::error::Error>>{
        if other.CRD_version != ""{
            if self.CRD_version == ""{
                self.CRD_version = other.CRD_version
            }
        }else if self.CRD_version != other.CRD_version {
            Ok(()) //return error
        }
        self.Namespaced = self.Namespaced || other.Namespaced;

    }
    fn is_empty(self) -> bool{
        self.CRD_version == "" && !self.Namespaced
    }
}