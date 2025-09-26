class logrotate < Formula
  desc "Slim version of a logrotate cli utility for:
        archiving, removing, and truncating files in a provided directory."
  homepage "https://github.com/jackpots28/logrotate"
  url "https://github.com/jackpots28/logrotate/releases/download/v0.1.0/logrotate-v0.1.1-mac.tar.gz"
  sha256 "cb681a88b362c2e75dcd00d26b893996b0c4274aec02317d55482dcacd57f7c4"

  def install
    bin.install "logrotate"
  end
end