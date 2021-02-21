#pragma once

#include <string>
#include <memory>
#include "rust/cxx.h"

namespace rango {

class Rango {
private:
  std::string dev_name;
public:
  Rango(const std::string &);
  double read_attribute(rust::Str) const;
};

std::unique_ptr<Rango> new_rango(rust::Str);

}
