"""Mock by default; --bridge requires UNO Q App Lab / APP_SOCKET router environment."""
import argparse,time
def mock(value,confidence,age):
 return value if value in (0,1) and 80<=confidence<=100 and 0<=age<=1000 else -1
def main():
 parser=argparse.ArgumentParser(); parser.add_argument('--bridge',action='store_true')
 args=parser.parse_args()
 if not args.bridge:
  cases=[(1,80,1000),(0,100,0),(1,79,0),(2,99,0),(1,101,0),(1,90,1001)]
  print('Mock policy only; no model inference, RPC or hardware claim.')
  print([mock(*case) for case in cases]); return
 from arduino.app_utils import Bridge
 try:
  ack=Bridge.call('review_led',1,90,timeout=1.0)
  actual=Bridge.call('read_led',timeout=1.0)
  print('Observed acceptance/state:',ack,actual)
  time.sleep(1.1) # MCU independently expires the1000ms lease.
  expired=Bridge.call('read_led',timeout=1.0)
  print('Observed state after expiry:',expired)
 except (TimeoutError,ConnectionError,ValueError) as error:
  print('Transport failure:',type(error).__name__,str(error))
  print('No retry/notify authorises control. MCU independently expires to safe OFF.')
if __name__=='__main__': main()
