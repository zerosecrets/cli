use crate::common::print_formatted_error::print_formatted_error;

pub struct Vendors {
    pub prettified_vendor_options: Vec<&'static str>,
}

impl Vendors {
    pub fn new() -> Self {
        Vendors {
            prettified_vendor_options: Vec::from([
                "Agora",
                "AWS",
                "Azure",
                "Braintree",
                "DigitalOcean",
                "GoogleCloud",
                "Mailchimp",
                "Mixpanel",
                "Other",
                "Paypal",
                "Pulumi",
                "Segment",
                "Sendgrid",
                "Stripe",
                "Terraform",
                "Twilio",
            ]),
        }
    }

    pub fn vendor_normalize(vendor: &str) -> &'static str {
        match vendor {
            "Agora" => "agora",
            "AWS" => "aws",
            "Azure" => "azure",
            "Braintree" => "braintree",
            "DigitalOcean" => "digitalOcean",
            "GoogleCloud" => "googleCloud",
            "Mailchimp" => "mailchimp",
            "Mixpanel" => "mixpanel",
            "Other" => "other",
            "Paypal" => "paypal",
            "Pulumi" => "pulumi",
            "Segment" => "segment",
            "Sendgrid" => "sendgrid",
            "Stripe" => "stripe",
            "Terraform" => "terraform",
            "Twilio" => "twilio",

            _ => {
                print_formatted_error("Service error. Please try again.");
                std::process::exit(1);
            }
        }
    }
}
