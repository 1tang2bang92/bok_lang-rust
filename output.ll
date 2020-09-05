; ModuleID = 'Entry'
source_filename = "Entry"

define i64 @fibo(i64 %a) {
entry:
  %a1 = alloca i64
  store i64 %a, i64* %a1
  %a2 = load i64, i64* %a1
  %comptmp = icmp eq i64 %a2, 0
  %ifcondition = icmp ne i1 %comptmp, false
  br i1 %ifcondition, label %then, label %else

then:                                             ; preds = %entry
  br label %murge

else:                                             ; preds = %entry
  %a3 = load i64, i64* %a1
  %subtmp = sub i64 %a3, 1
  %calltmp = call i64 @fibo(i64 %subtmp)
  %a4 = load i64, i64* %a1
  %multmp = mul i64 %calltmp, %a4
  br label %murge

murge:                                            ; preds = %else, %then
  %iftmp = phi i64 [ %multmp, %else ], [ 1, %then ]
  ret i64 %iftmp
}
