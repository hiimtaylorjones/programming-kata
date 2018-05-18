class CheckOut

  attr_reader :rules
  attr_accessor :total

  def initialize(rules)
    @rules = rules
    @total = 0
  end

  def scan(item)
    return if item == ""
    @total = @total + find_price(item)
  end

private

  def find_price(item)
    return @rules[item.to_sym][:price]
  end

  def check_for_special(item)
    return @rules[item.to_sym].has_key?(:special)
  end

end
