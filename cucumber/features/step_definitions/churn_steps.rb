Given("there is a repo with commits") do |table|
  table.hashes.each_with_index do |commit, index|
    (commit["Add"].split + commit["Modify"].split).each { |file| change_file(file) }
    commit["Delete"].split.each { |file| delete_file(file) }
    add_commit(index)
  end
end

When("churn analyze the repo") do
  path = ENV["BIN_PATH"] || "../cli/target/debug/vcanrs"
  @report = shell "#{path} churn #{@path}"
end

Then("the report shows") do |table|
  result = @report.scan(/\|\s*(.*?)\s*\|\s*(\d+)\s*\|/)
  expect(result.size).to eq(table.hashes.size)
  table.hashes.each_with_index do |expected, index|
    expect(result[index][0]).to eq(expected["file"])
    expect(result[index][1]).to eq(expected["churn"])
  end
end

And(/^rename file "([^"]*)" to "([^"]*)"$/) do |old_file, new_file|
  move_file old_file, new_file
  add_commit(5)
end
