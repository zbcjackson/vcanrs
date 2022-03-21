def shell(cmd)
  output = `#{cmd}`
  unless $?.success?
    p output
    raise "Shell error"
  end
  output
end

def add_dir(dir)
  Dir.mkdir("#{@path}/#{dir}")
end

def change_file(file)
  shell "echo #{rand} >> #{@path}/#{file}"
end

def move_file(file, to)
  last_slash = to.rindex("/")
  if last_slash
    dirs = to[0..last_slash - 1]
    shell "mkdir -p #{@path}/#{dirs}"
  end
  shell "mv #{@path}/#{file} #{@path}/#{to}"
end

def delete_file(file)
  shell "rm #{@path}/#{file}"
end
