require_relative "../shell/shell"
require_relative "../shell/git"

Before do
  init_repo
end

After do |scenario|
  delete_repo
end
