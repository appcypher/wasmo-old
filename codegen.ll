; ModuleID = 'wasm'
source_filename = "wasm"

%InstanceContext = type { i8 addrspace(1)* addrspace(1)*, %BoundPtr addrspace(1)*, i64 addrspace(1)* addrspace(1)*, i8 addrspace(1)* addrspace(1)* }
%BoundPtr = type { i32 addrspace(1)*, i64 }

define void @0(%InstanceContext addrspace(1)*) {
entry:
  ret void
}

define i64 @1(%InstanceContext addrspace(1)*, i64) {
entry:
  ret i64 %1
}

define i64 @2(%InstanceContext addrspace(1)*, i64, i64) {
entry:
  %i64.mul = mul i64 %2, %1
  ret i64 %i64.mul
}

define float @3(%InstanceContext addrspace(1)*, float) {
entry:
  %f32.mul = fmul float 0.000000e+00, %1
  ret float %f32.mul
}

define double @4(%InstanceContext addrspace(1)*, double, double, double) {
entry:
  %f64.add = fadd double %3, %2
  %f64.add1 = fadd double %f64.add, %1
  ret double %f64.add1
}

define i32 @5(%InstanceContext addrspace(1)*) {
entry:
  ret i32 700
}

define void @main(...) {
entry:
  ret void
}
