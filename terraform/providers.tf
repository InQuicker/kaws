provider "aws" {
  region = "${var.region}"
  version = "> 0.1"
}

provider "template" {
  version = "> 0.1"
}
