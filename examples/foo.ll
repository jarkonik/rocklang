; ModuleID = 'main'
source_filename = "main"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

define void @__main__() {
entry:
  %i = alloca double, align 8
  store double 1.000000e+00, double* %i, align 8
  %0 = load double, double* %i, align 8
  %1 = fcmp olt double %0, 2.000000e+01
  br i1 %1, label %loop, label %afterloop

loop:                                             ; preds = %loop, %entry
  br label %afterloop
  ret void

afterloop:                                        ; preds = %loop, %loop, %entry
  ret void
}