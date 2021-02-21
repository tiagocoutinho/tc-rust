#include <iostream>
#include "cpp-rust/include/rango.h"

namespace rango {

Rango::Rango(const std::string &name): dev_name(name) {}

double Rango::read_attribute(rust::Str attr_name) const {
  std::cout << "Read '" << attr_name << "' from '" << this->dev_name << "'" << std::endl;
  return 1.2345;
}

std::unique_ptr<Rango> new_rango(rust::Str dev_name) {
  return std::make_unique<Rango>(std::string(dev_name));
}

}
