use crate::common::print_formatted_error::print_formatted_error;

pub struct Vendors {
    pub prettified_vendor_options: Vec<&'static str>,
}

impl Vendors {
    pub fn new() -> Self {
        Vendors {
            prettified_vendor_options: Vec::from([
                "Agora",
                "Ansible",
                "AWS",
                "Azure",
                "Bitbucket",
                "Braintree",
                "Claude",
                "Datadog",
                "DigitalOcean",
                "Docker",
                "Facebook",
                "Gemini",
                "GitHub",
                "GitLab",
                "Google",
                "GoogleCloud",
                "Jenkins",
                "Jira",
                "Kubernetes",
                "Linear",
                "Mailchimp",
                "Mixpanel",
                "OpenAI",
                "Other",
                "Paypal",
                "Pulumi",
                "Salesforce",
                "Segment",
                "Sendgrid",
                "Shopify",
                "Slack",
                "Stripe",
                "Terraform",
                "Trello",
                "Twilio",
                "Zoom",
            ]),
        }
    }

    pub fn vendor_normalize(vendor: &str) -> &'static str {
        match vendor {
            "Agora" => "agora",
            "Ansible" => "ansible",
            "AWS" => "aws",
            "Azure" => "azure",
            "Bitbucket" => "bitbucket",
            "Braintree" => "braintree",
            "Claude" => "claude",
            "Datadog" => "datadog",
            "DigitalOcean" => "digitalOcean",
            "Docker" => "docker",
            "Facebook" => "facebook",
            "Gemini" => "gemini",
            "GitHub" => "gitHub",
            "GitLab" => "gitLab",
            "Google" => "google",
            "GoogleCloud" => "googleCloud",
            "Jenkins" => "jenkins",
            "Jira" => "jira",
            "Kubernetes" => "kubernetes",
            "Linear" => "linear",
            "Mailchimp" => "mailchimp",
            "Mixpanel" => "mixpanel",
            "OpenAI" => "openAI",
            "Other" => "other",
            "Paypal" => "paypal",
            "Pulumi" => "pulumi",
            "Salesforce" => "salesforce",
            "Segment" => "segment",
            "Sendgrid" => "sendgrid",
            "Shopify" => "shopify",
            "Slack" => "slack",
            "Stripe" => "stripe",
            "Terraform" => "terraform",
            "Trello" => "trello",
            "Twilio" => "twilio",
            "Zoom" => "zoom",
            _ => {
                print_formatted_error("Service error. Please try again.");
                std::process::exit(1);
            }
        }
    }
}
