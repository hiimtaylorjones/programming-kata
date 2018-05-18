class CheckOut

  attr_reader :rules
  attr_accessor :total

  def initialize(rules)
    @rules = rules
    @total = 0
    @order = ""
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
      @rules[symbol][:special][:units] == @order.count(item)
  end

  def apply_special(item)
    symbol = item.to_sym
    @total = @total - ((@order.count(item) - 1) * @rules[symbol][:price])
    @total = @total + @rules[symbol][:special][:price]
  end

end
