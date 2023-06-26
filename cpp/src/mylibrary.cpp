#include "mylibrary.h"
#include <casadi/casadi.hpp>
#include <casadi/core/optistack.hpp>

double my_special_function(double input) {
  casadi::Opti opti;
  opti.solver("ipopt");
  casadi::MX x = opti.variable();
  opti.minimize((x + input) * (x + input));
  opti.solve();
  return static_cast<double>(opti.value(x));
}
