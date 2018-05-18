class CheckOut

  attr_reader :rules
  attr_accessor :total

  def initialize(rules)
    @rules = rules
    @total = 0
    @order = ""
    puts "\nNew Checkout Instance!"
  end

  def scan(item)
    return if item == ""
    @order = @order + item
    add_price(item)
  end

private

  def add_price(item)
    if check_for_special(item) == true
      apply_special(item)
    else
      @total = @total + @rules[item.to_sym][:price]
    end
  end

  def check_for_special(item)
    symbol = item.to_sym
    @rules[symbol].has_key?(:special) &&
      @order.count(item) > 1 &&
      ((@order.count(item) % @rules[symbol][:special][:units]) == 0)
  end

  def apply_special(item)
    symbol = item.to_sym
    subtracting = ((@rules[symbol][:special][:units] - 1) * @rules[symbol][:price])
    puts "Subtracting #{subtracting} from total"
    @total = @total - subtracting
    puts "Previous total: #{@total}"
    special_price = @rules[symbol][:special][:price]
    puts "Adding special price of #{special_price}"
    @total = @total + special_price
    puts "New total: #{@total}"
  end

end
