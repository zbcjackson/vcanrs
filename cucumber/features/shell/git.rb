require "fileutils"
require "tmpdir"

def init_repo
  Dir.mkdir "./tmp" unless File.exist?("./tmp")
  dir = Dir.mktmpdir
  @path = "#{dir}/repo#{Time.now.to_i}"
  p @path
  Dir.mkdir @path
  Dir.chdir @path do
    shell "git init"
  end
end

def add_commit(index)
  Dir.chdir @path do
    shell "git add ."
    shell "git commit -m 'commit #{index}'"
  end
end

def delete_repo
  FileUtils.rm_rf @path
end
