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
                "Ansible",
                "Bitbucket",
                "Claude",
                "Datadog",
                "Docker",
                "Facebook",
                "Gemini",
                "GitHub",
                "GitLab",
                "Google",
                "Jenkins",
                "Jira",
                "Kubernetes",
                "Linear",
                "OpenAI",
                "Salesforce",
                "Shopify",
                "Slack",
                "Trello",
                "Zoom",
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
            "Ansible" => "ansible",
            "Bitbucket" => "bitbucket",
            "Claude" => "claude",
            "Datadog" => "datadog",
            "Docker" => "docker",
            "Facebook" => "facebook",
            "Gemini" => "gemini",
            "GitHub" => "gitHub",
            "GitLab" => "gitLab",
            "Google" => "google",
            "Jenkins" => "jenkins",
            "Jira" => "jira",
            "Kubernetes" => "kubernetes",
            "Linear" => "linear",
            "OpenAI" => "openAI",
            "Salesforce" => "salesforce",
            "Shopify" => "shopify",
            "Slack" => "slack",
            "Trello" => "trello",
            "Zoom" => "zoom",
            _ => {
                print_formatted_error("Service error. Please try again.");
                std::process::exit(1);
            }
        }
    }
}
